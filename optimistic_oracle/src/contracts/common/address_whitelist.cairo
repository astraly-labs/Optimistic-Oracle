#[starknet::contract]
pub mod address_whitelist {
    use core::starknet::event::EventEmitter;
    use starknet::{storage_access::{Store, StorageBaseAddress}, SyscallResult, ContractAddress};
    use openzeppelin::security::reentrancyguard::{
        ReentrancyGuardComponent,
        ReentrancyGuardComponent::InternalTrait as InternalReentrancyGuardImpl
    };
    use starknet::ClassHash;
    use optimistic_oracle::contracts::interfaces::IAddressWhitelist;
    use openzeppelin::access::ownable::OwnableComponent;
    use openzeppelin::upgrades::{interface::IUpgradeable, upgradeable::UpgradeableComponent};
    use core::hash::{LegacyHash, HashStateTrait};
    component!(path: OwnableComponent, storage: ownable, event: OwnableEvent);
    component!(
        path: ReentrancyGuardComponent, storage: reentrancy_guard, event: ReentrancyGuardEvent
    );
    component!(path: UpgradeableComponent, storage: upgradeable, event: UpgradeableEvent);
    #[abi(embed_v0)]
    impl OwnableImpl = OwnableComponent::OwnableImpl<ContractState>;
    impl OwnableInternalImpl = OwnableComponent::InternalImpl<ContractState>;
    impl UpgradeableInternalImpl = UpgradeableComponent::InternalImpl<ContractState>;


    #[derive(PartialEq, Drop, Serde)]
    pub enum Status {
        None,
        Out,
        In,
    }

    #[derive(PartialEq, Drop, Serde, starknet::Store, Copy)]
    pub enum WhitelistType {
        Currency,
        User,
    }

    // Implement LegacyHash for (WhitelistType, ContractAddress)
    impl WhitelistKeyHash of LegacyHash<(WhitelistType, ContractAddress)> {
        fn hash(state: felt252, value: (WhitelistType, ContractAddress)) -> felt252 {
            let (whitelist_type, address) = value;
            let whitelist_type_felt252 = match whitelist_type {
                WhitelistType::Currency => { 0 },
                WhitelistType::User => { 1 }
            };
            let mut state = LegacyHash::<felt252>::hash(state, whitelist_type_felt252);
            LegacyHash::<ContractAddress>::hash(state, address)
        }
    }


    // Store manual implementation (basic implementation panics if no default enum is defined for a given address, cf: https://github.com/starkware-libs/cairo/blob/b741c26c553fd9fa3246cee91fd5c637f225cdb9/crates/cairo-lang-starknet/src/plugin/derive/store.rs#L263)
    impl StatusStoreImpl of Store<Status> {
        fn read(address_domain: u32, base: StorageBaseAddress) -> SyscallResult<Status> {
            match Store::<felt252>::read(address_domain, base)? {
                0 => Result::Ok(Status::None),
                1 => Result::Ok(Status::Out),
                2 => Result::Ok(Status::In),
                _ => SyscallResult::Err(array!['Invalid Status value']),
            }
        }
        fn write(
            address_domain: u32, base: StorageBaseAddress, value: Status
        ) -> SyscallResult<()> {
            let value_felt = match value {
                Status::None => 0,
                Status::Out => 1,
                Status::In => 2,
            };
            Store::<felt252>::write(address_domain, base, value_felt)
        }

        fn read_at_offset(
            address_domain: u32, base: StorageBaseAddress, offset: u8
        ) -> SyscallResult<Status> {
            match Store::<felt252>::read_at_offset(address_domain, base, offset)? {
                0 => SyscallResult::Ok(Status::None),
                1 => SyscallResult::Ok(Status::Out),
                2 => SyscallResult::Ok(Status::In),
                _ => SyscallResult::Err(array!['Invalid Status value']),
            }
        }
        fn write_at_offset(
            address_domain: u32, base: StorageBaseAddress, offset: u8, value: Status
        ) -> SyscallResult<()> {
            let value_felt = match value {
                Status::None => 0,
                Status::Out => 1,
                Status::In => 2,
            };
            Store::<felt252>::write_at_offset(address_domain, base, offset, value_felt)
        }
        fn size() -> u8 {
            1
        }
    }


    #[event]
    #[derive(Drop, starknet::Event)]
    pub enum Event {
        AddedToWhitelist: AddedToWhitelist,
        RemovedFromWhitelist: RemovedFromWhitelist,
        #[flat]
        OwnableEvent: OwnableComponent::Event,
        #[flat]
        ReentrancyGuardEvent: ReentrancyGuardComponent::Event,
        #[flat]
        UpgradeableEvent: UpgradeableComponent::Event,
    }


    #[derive(starknet::Event, Drop)]
    pub struct AddedToWhitelist {
        pub added_address: ContractAddress,
        pub whitelist_type: WhitelistType,
    }

    #[derive(starknet::Event, Drop)]
    pub struct RemovedFromWhitelist {
        pub removed_address: ContractAddress,
        pub whitelist_type: WhitelistType,
    }

    #[storage]
    struct Storage {
        whitelist_indices: LegacyMap::<(WhitelistType, ContractAddress), ContractAddress>,
        whitelist: LegacyMap::<(WhitelistType, ContractAddress), Status>,
        #[substorage(v0)]
        ownable: OwnableComponent::Storage,
        #[substorage(v0)]
        reentrancy_guard: ReentrancyGuardComponent::Storage,
        #[substorage(v0)]
        upgradeable: UpgradeableComponent::Storage,
    }


    #[constructor]
    fn constructor(ref self: ContractState, owner: ContractAddress) {
        self.ownable.initializer(owner);
    }

    #[abi(embed_v0)]
    impl Upgradeable of IUpgradeable<ContractState> {
        /// Upgrades the contract to a new implementation.
        /// Callable only by the owner
        /// # Arguments
        ///
        /// * `new_class_hash` - The class hash of the new implementation.
        fn upgrade(ref self: ContractState, new_class_hash: ClassHash) {
            self.ownable.assert_only_owner();
            self.upgradeable.upgrade(new_class_hash);
        }
    }

    #[abi(embed_v0)]
    impl IAddressWhitelistImpl of IAddressWhitelist<ContractState> {
        fn add_to_whitelist(
            ref self: ContractState, new_element: ContractAddress, whitelist_type: WhitelistType
        ) {
            self.ownable.assert_only_owner();
            self.reentrancy_guard.start();
            match self.get_status(new_element, whitelist_type) {
                Option::Some(status) => {
                    if (status == Status::In) {
                        self.reentrancy_guard.end();
                        return;
                    } else if (status == Status::Out) {
                        self.whitelist.write((whitelist_type, new_element), Status::In);
                        self.emit(AddedToWhitelist { added_address: new_element, whitelist_type });
                    } else {
                        self.insert_to_whitelist(new_element, whitelist_type);
                        self.whitelist.write((whitelist_type, new_element), Status::In);
                        self.emit(AddedToWhitelist { added_address: new_element, whitelist_type });
                    }
                },
                Option::None => {
                    self.insert_to_whitelist(new_element, whitelist_type);
                    self.whitelist.write((whitelist_type, new_element), Status::In);
                    self.emit(AddedToWhitelist { added_address: new_element, whitelist_type });
                }
            }
            self.reentrancy_guard.end();
        }

        fn remove_from_whitelist(
            ref self: ContractState,
            element_to_remove: ContractAddress,
            whitelist_type: WhitelistType
        ) {
            self.ownable.assert_only_owner();
            self.reentrancy_guard.start();
            if (self.whitelist.read((whitelist_type, element_to_remove)) != Status::Out) {
                self.whitelist.write((whitelist_type, element_to_remove), Status::Out);
                self
                    .emit(
                        RemovedFromWhitelist { removed_address: element_to_remove, whitelist_type }
                    );
            }
            self.reentrancy_guard.end();
        }

        fn is_on_whitelist(
            self: @ContractState, element_to_check: ContractAddress, whitelist_type: WhitelistType
        ) -> bool {
            self.whitelist.read((whitelist_type, element_to_check)) == Status::In
        }

        fn get_whitelist(
            self: @ContractState, whitelist_type: WhitelistType
        ) -> Span<ContractAddress> {
            self.build_whitelist_indices_array(whitelist_type)
        }
    }

    #[generate_trait]
    impl InternalTraitImpl of InternalTrait {
        fn get_status(
            self: @ContractState, address: ContractAddress, whitelist_type: WhitelistType
        ) -> Option<Status> {
            match self.whitelist.read((whitelist_type, address)) {
                Status::None => Option::Some(Status::None),
                Status::In => Option::Some(Status::In),
                Status::Out => Option::Some(Status::Out),
                _ => Option::None,
            }
        }

        fn find_last_whitelist_indice(
            self: @ContractState, whitelist_type: WhitelistType
        ) -> ContractAddress {
            let mut current_indice = self
                .whitelist_indices
                .read((whitelist_type, 0.try_into().unwrap()));
            loop {
                let next_indice = self.whitelist_indices.read((whitelist_type, current_indice));
                if next_indice == 0.try_into().unwrap() {
                    break current_indice;
                }
                current_indice = next_indice;
            }
        }

        fn build_whitelist_indices_array(
            self: @ContractState, whitelist_type: WhitelistType
        ) -> Span<ContractAddress> {
            let mut index = 0.try_into().unwrap();
            let mut whitelist_indices = array![];
            loop {
                let indice = self.whitelist_indices.read((whitelist_type, index));
                if (indice == 0.try_into().unwrap()) {
                    break ();
                }
                if (self.whitelist.read((whitelist_type, indice)) == Status::In) {
                    whitelist_indices.append(indice);
                }
                index = indice;
            };

            whitelist_indices.span()
        }

        fn insert_to_whitelist(
            ref self: ContractState, new_element: ContractAddress, whitelist_type: WhitelistType
        ) {
            let last_index = self.find_last_whitelist_indice(whitelist_type);
            self.whitelist_indices.write((whitelist_type, last_index), new_element);
        }
    }
}
