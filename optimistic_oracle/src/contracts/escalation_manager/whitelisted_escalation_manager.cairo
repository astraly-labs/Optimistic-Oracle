#[starknet::contract]
pub mod whitelisted_escalation_manager {
    use starknet::ContractAddress;
    use optimistic_oracle::contracts::interfaces::{
        IOptimisticOracleDispatcher, IOptimisticOracleDispatcherTrait, AssertionPolicy,
        IEscalationManager, IDisputeCallerConfiguration
    };
    use openzeppelin::access::ownable::OwnableComponent;

    component!(path: OwnableComponent, storage: ownable, event: OwnableEvent);
    #[abi(embed_v0)]
    impl OwnableImpl = OwnableComponent::OwnableImpl<ContractState>;
    impl OwnableInternalImpl = OwnableComponent::InternalImpl<ContractState>;


    #[storage]
    struct Storage {
        optimistic_oracle: IOptimisticOracleDispatcher,
        whitelisted_dispute_callers: LegacyMap::<ContractAddress, bool>,
        #[substorage(v0)]
        ownable: OwnableComponent::Storage,
    }


    #[derive(starknet::Event, Drop)]
    pub struct PriceRequestAdded {
        pub identifier: felt252,
        pub time: u256,
        pub ancillary_data: ByteArray
    }


    #[event]
    #[derive(Drop, starknet::Event)]
    enum Event {
        PriceRequestAdded: PriceRequestAdded,
        #[flat]
        OwnableEvent: OwnableComponent::Event,
    }

    #[constructor]
    fn constructor(
        ref self: ContractState, owner: ContractAddress, optimistic_oracle: ContractAddress,
    ) {
        self.ownable.initializer(owner);
        self
            .optimistic_oracle
            .write(IOptimisticOracleDispatcher { contract_address: optimistic_oracle });
    }

    impl IWhitelistedEscalationManagerImpl of IEscalationManager<ContractState> {
        fn get_assertion_policy(self: @ContractState, assertion_id: felt252) -> AssertionPolicy {
            AssertionPolicy {
                block_assertion: false,
                arbitrate_via_escalation_manager: false,
                discard_oracle: false,
                validate_disputers: true
            }
        }
        fn is_dispute_allowed(
            self: @ContractState, assertion_id: felt252, dispute_caller: ContractAddress
        ) -> bool {
            // TODO: verify that the bool legacy map returns false if the dispute caller is not registered.
            self.whitelisted_dispute_callers.read(dispute_caller)
        }
        fn get_price(
            self: @ContractState, identifier: felt252, time: u256, ancillary_data: ByteArray
        ) -> u256 {
            0
        }

        fn request_price(
            ref self: ContractState, identifier: felt252, time: u256, ancillary_data: ByteArray
        ) -> felt252 {
            self.emit(PriceRequestAdded { identifier, time, ancillary_data });
            0
        }
    }

    #[abi(embed_v0)]
    impl IDisputeCallerConfigurationImpl of IDisputeCallerConfiguration<ContractState> {
        fn set_dispute_caller_in_whitelist(
            ref self: ContractState, dispute_caller: ContractAddress, value: bool
        ) {
            self.ownable.assert_only_owner();
            self.whitelisted_dispute_callers.write(dispute_caller, value);
        }
    }
}
