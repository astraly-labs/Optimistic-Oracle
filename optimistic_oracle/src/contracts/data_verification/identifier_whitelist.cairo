#[starknet::contract]
pub mod identifier_whitelist {
    use core::starknet::event::EventEmitter;
    use openzeppelin::access::ownable::ownable::OwnableComponent::InternalTrait;
    use optimistic_oracle::contracts::interfaces::IIdentifierWhitelist;
    use openzeppelin::access::ownable::OwnableComponent;
    component!(path: OwnableComponent, storage: ownable, event: OwnableEvent);
    #[abi(embed_v0)]
    impl OwnableImpl = OwnableComponent::OwnableImpl<ContractState>;
    impl OwnableInternalImpl = OwnableComponent::InternalImpl<ContractState>;


    #[storage]
    struct Storage {
        supported_identifiers: LegacyMap::<felt252, bool>,
        #[substorage(v0)]
        ownable: OwnableComponent::Storage,
    }


    #[event]
    #[derive(Drop, starknet::Event)]
    enum Event {
        SupportedIdentifierAdded: SupportedIdentifierAdded,
        SupportedIdentifierRemoved: SupportedIdentifierRemoved,
        #[flat]
        OwnableEvent: OwnableComponent::Event,
    }


    #[derive(starknet::Event, Drop)]
    pub struct SupportedIdentifierAdded {
        pub identifier: felt252,
    }

    #[derive(starknet::Event, Drop)]
    pub struct SupportedIdentifierRemoved {
        pub identifier: felt252,
    }


    impl IIdentifierWhitelistImpl of IIdentifierWhitelist<ContractState> {
        fn add_supported_identifier(ref self: ContractState, identifier: felt252) {
            self.ownable.assert_only_owner();
            if (!self.supported_identifiers.read(identifier)) {
                self.supported_identifiers.write(identifier, true);
            }
            self.emit(SupportedIdentifierAdded { identifier });
        }

        fn remove_supported_identifier(ref self: ContractState, identifier: felt252) {
            self.ownable.assert_only_owner();
            if (self.supported_identifiers.read(identifier)) {
                self.supported_identifiers.write(identifier, false);
            }
            self.emit(SupportedIdentifierRemoved { identifier });
        }

        fn is_identifier_supported(self: @ContractState, identifier: felt252) -> bool {
            self.supported_identifiers.read(identifier)
        }
    }
}
