#[starknet::contract]
pub mod address_whitelist {
    use core::starknet::event::EventEmitter;
    use starknet::{storage_access::{Store, StorageBaseAddress},SyscallResult, ContractAddress};
    use openzeppelin::security::reentrancyguard::{
        ReentrancyGuardComponent,
        ReentrancyGuardComponent::InternalTrait as InternalReentrancyGuardImpl
    };
    use optimistic_oracle::contracts::interfaces::IAddressWhitelist;
    use openzeppelin::access::ownable::OwnableComponent;
    component!(path: OwnableComponent, storage: ownable, event: OwnableEvent);
    #[abi(embed_v0)]
    impl OwnableImpl = OwnableComponent::OwnableImpl<ContractState>;
    impl OwnableInternalImpl = OwnableComponent::InternalImpl<ContractState>;
    component!(
        path: ReentrancyGuardComponent, storage: reentrancy_guard, event: ReentrancyGuardEvent
    );

    #[derive(PartialEq, Drop, Serde)]
    pub enum Status {
        None, 
        Out,
        In,
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
                fn write(address_domain: u32, base: StorageBaseAddress, value: Status) -> SyscallResult<()> {
                    let value_felt = match value {
                                Status::None => 0,
                                Status::Out => 1,
                                Status::In => 2,
                            };
                        Store::<felt252>::write(address_domain, base, value_felt)
        }

        fn read_at_offset(address_domain: u32, base: StorageBaseAddress, offset:u8) -> SyscallResult<Status> {
            match Store::<felt252>::read_at_offset(address_domain, base, offset)? {
                0 => SyscallResult::Ok(Status::None),
                1 => SyscallResult::Ok(Status::Out),
                2 => SyscallResult::Ok(Status::In),
                _ => SyscallResult::Err(array!['Invalid Status value']),
            }
        }
        fn write_at_offset(address_domain: u32, base: StorageBaseAddress, offset: u8,value: Status) -> SyscallResult<()> {
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
    }


    #[derive(starknet::Event, Drop)]
    pub struct AddedToWhitelist {
        pub added_address: ContractAddress,
    }
    #[derive(starknet::Event, Drop)]
    pub struct RemovedFromWhitelist {
        pub removed_address: ContractAddress,
    }

    #[storage]
    struct Storage {
        whitelist_indices: LegacyMap::<ContractAddress, ContractAddress>,
        whitelist: LegacyMap::<ContractAddress, Status>,
        #[substorage(v0)]
        ownable: OwnableComponent::Storage,
        #[substorage(v0)]
        reentrancy_guard: ReentrancyGuardComponent::Storage,
    }


    #[constructor]
    fn constructor(ref self: ContractState, owner: ContractAddress) {
        self.ownable.initializer(owner);
    }

    #[abi(embed_v0)]
    impl IAddressWhitelistImpl of IAddressWhitelist<ContractState> {
        fn add_to_whitelist(ref self: ContractState, new_element: ContractAddress) {
            self.ownable.assert_only_owner();
            self.reentrancy_guard.start();
            match self.get_status(new_element) {
                Option::Some(status) => {
                    if (status == Status::In){
                        self.reentrancy_guard.end();
                        return;
                    } else if (status ==Status::Out){
                        self.whitelist.write(new_element, Status::In);
                        self.emit(AddedToWhitelist { added_address: new_element }); 
                    } else {
                        self.insert_to_whitelist(new_element);
                        self.whitelist.write(new_element, Status::In);
                        self.emit(AddedToWhitelist { added_address: new_element });
                    }
                },
                Option::None => {
                    self.insert_to_whitelist(new_element);
                    self.whitelist.write(new_element, Status::In);
                    self.emit(AddedToWhitelist { added_address: new_element });
                }
            }
            self.reentrancy_guard.end();
        }

        fn remove_from_whitelist(ref self: ContractState, element_to_remove: ContractAddress) {
            self.ownable.assert_only_owner();
            self.reentrancy_guard.start();
            if (self.whitelist.read(element_to_remove) != Status::Out) {
                self.whitelist.write(element_to_remove, Status::Out);
                self.emit(RemovedFromWhitelist { removed_address: element_to_remove });
            }
            self.reentrancy_guard.end();
        }

        fn is_on_whitelist(self: @ContractState, element_to_check: ContractAddress) -> bool {
            self.whitelist.read(element_to_check) == Status::In
        }

        fn get_whitelist(self: @ContractState) -> Span<ContractAddress> {
            self.build_whitelist_indices_array()
        }
    }

    #[generate_trait]
    impl InternalTraitImpl of InternalTrait {


        fn get_status(self: @ContractState, address: ContractAddress) -> Option<Status> {
            match self.whitelist.read(address) {
                Status::None => Option::Some(Status::None),
                Status::In => Option::Some(Status::In),
                Status::Out => Option::Some(Status::Out),
                _ => Option::None,
            }
        }


        fn find_last_whitelist_indice(self: @ContractState) -> ContractAddress {
            let mut current_indice = self.whitelist_indices.read(0.try_into().unwrap());
            loop {
                let next_indice = self.whitelist_indices.read(current_indice);
                if next_indice == 0.try_into().unwrap() {
                    break current_indice;
                }
                current_indice = next_indice;
            }
        }

        // Helper: builds a span of whitelist_indices from the storage map
        fn build_whitelist_indices_array(self: @ContractState) -> Span<ContractAddress> {
            let mut index = 0.try_into().unwrap();
            let mut whitelist_indices = array![];
            loop {
                let indice = self.whitelist_indices.read(index);
                if (indice == 0.try_into().unwrap()) {
                    break ();
                }
                if (self.whitelist.read(indice) == Status::In) {
                    whitelist_indices.append(indice);
                }
                index = indice;
            };

            whitelist_indices.span()
        }

        fn insert_to_whitelist(ref self: ContractState, new_element: ContractAddress) {
            let last_index = self.find_last_whitelist_indice();
            self.whitelist_indices.write(last_index, new_element);
        }
    }
}
