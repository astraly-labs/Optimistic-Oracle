#[starknet::contract]
pub mod base_escalation_manager {
    use starknet::ContractAddress;
    use optimistic_oracle::contracts::interfaces::{
        IOptimisticOracleDispatcher, IOptimisticOracleDispatcherTrait, AssertionPolicy,
        IEscalationManager, IAssertionCallback
    };

    #[storage]
    struct Storage {
        optimistic_oracle: IOptimisticOracleDispatcher,
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
    }
    #[constructor]
    fn constructor(ref self: ContractState, optimistic_oracle: ContractAddress,) {
        self
            .optimistic_oracle
            .write(IOptimisticOracleDispatcher { contract_address: optimistic_oracle });
    }

    #[abi(embed_v0)]
    impl IBaseEscalationManagerImpl of IEscalationManager<ContractState> {
        fn get_assertion_policy(self: @ContractState, assertion_id: felt252,) -> AssertionPolicy {
            AssertionPolicy {
                block_assertion: false,
                arbitrate_via_escalation_manager: false,
                discard_oracle: false,
                validate_disputers: false
            }
        }

        fn is_dispute_allowed(
            self: @ContractState, assertion_id: felt252, dispute_caller: ContractAddress
        ) -> bool {
            true
        }

        fn get_price(
            self: @ContractState, identifier: felt252, time: u256, ancillary_data: ByteArray
        ) -> u256 {
            0
        }

        fn request_price(
            ref self: ContractState, identifier: felt252, time: u256, ancillary_data: ByteArray
        ) {
            self.emit(PriceRequestAdded { identifier, time, ancillary_data });
        }
    }


    #[abi(embed_v0)]
    impl CallbackImpl of IAssertionCallback<ContractState> {
        fn assertion_resolved_callback(
            self: @ContractState, assertion_id: felt252, asserted_truthfully: bool
        ) {}

        fn assertion_disputed_callback(self: @ContractState, assertion_id: felt252) {}
    }
}
