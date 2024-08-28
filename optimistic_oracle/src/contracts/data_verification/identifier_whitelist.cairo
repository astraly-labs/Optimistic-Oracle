/// Store whitelist of supported identifiers, unique piece of information that the oracle provide result for. 

#[starknet::contract]
pub mod identifier_whitelist {
    use starknet::{ContractAddress, ClassHash};
    use core::starknet::event::EventEmitter;
    use openzeppelin::access::ownable::ownable::OwnableComponent::InternalTrait;
    use optimistic_oracle::contracts::interfaces::IIdentifierWhitelist;
    use openzeppelin::access::ownable::OwnableComponent;
    use openzeppelin::upgrades::{interface::IUpgradeable, upgradeable::UpgradeableComponent};
    component!(path: OwnableComponent, storage: ownable, event: OwnableEvent);
    component!(path: UpgradeableComponent, storage: upgradeable, event: UpgradeableEvent);
    #[abi(embed_v0)]
    impl OwnableImpl = OwnableComponent::OwnableImpl<ContractState>;
    impl OwnableInternalImpl = OwnableComponent::InternalImpl<ContractState>;
    impl UpgradeableInternalImpl = UpgradeableComponent::InternalImpl<ContractState>;


    #[storage]
    struct Storage {
        supported_identifiers: LegacyMap::<felt252, bool>,
        #[substorage(v0)]
        ownable: OwnableComponent::Storage,
        #[substorage(v0)]
        upgradeable: UpgradeableComponent::Storage,
    }


    #[event]
    #[derive(Drop, starknet::Event)]
    pub enum Event {
        SupportedIdentifierAdded: SupportedIdentifierAdded,
        SupportedIdentifierRemoved: SupportedIdentifierRemoved,
        #[flat]
        OwnableEvent: OwnableComponent::Event,
        #[flat]
        UpgradeableEvent: UpgradeableComponent::Event,
    }


    #[derive(starknet::Event, Drop)]
    pub struct SupportedIdentifierAdded {
        pub identifier: felt252,
    }

    #[derive(starknet::Event, Drop)]
    pub struct SupportedIdentifierRemoved {
        pub identifier: felt252,
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
    impl IIdentifierWhitelistImpl of IIdentifierWhitelist<ContractState> {
        /// Add the provided identifier as supported identifier 
        /// Dev: callable only by the admin 
        /// 
        /// # Arguments 
        /// 
        /// * `identifier` - an unique representation of the feed being added
        fn add_supported_identifier(ref self: ContractState, identifier: felt252) {
            self.ownable.assert_only_owner();
            if (!self.supported_identifiers.read(identifier)) {
                self.supported_identifiers.write(identifier, true);
                self.emit(SupportedIdentifierAdded { identifier });
            }
        }

        /// Remove the provided identifier as supported identifier 
        /// Dev: callable only by the admin 
        /// 
        /// # Arguments 
        /// 
        /// * `identifier` - an unique representation of the feed being removed
        fn remove_supported_identifier(ref self: ContractState, identifier: felt252) {
            self.ownable.assert_only_owner();
            if (self.supported_identifiers.read(identifier)) {
                self.supported_identifiers.write(identifier, false);
                self.emit(SupportedIdentifierRemoved { identifier });
            }
        }

        /// Check if a given identifier is supported or not.  
        /// 
        /// # Arguments 
        /// 
        /// * `identifier` - the identifier to check the information for
        fn is_identifier_supported(self: @ContractState, identifier: felt252) -> bool {
            self.supported_identifiers.read(identifier)
        }
    }
}
