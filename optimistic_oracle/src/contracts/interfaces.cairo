use core::byte_array::{ByteArray, ByteArrayTrait};
use starknet::ContractAddress;
use openzeppelin::token::erc20::interface::ERC20ABIDispatcher;
use optimistic_oracle::contracts::mocks::oracle_ancillary::mock_oracle_ancillary::{
    QueryPoint, QueryIndex
};

#[derive(starknet::Store, Drop, Serde)]
pub struct EscalationManagerSettings {
    pub arbitrate_via_escalation_manager: bool,
    pub discard_oracle: bool,
    pub validate_disputers: bool,
    pub asserting_caller: ContractAddress,
    pub escalation_manager: ContractAddress,
}

#[derive(starknet::Store, Drop, Serde)]
pub struct Assertion {
    pub escalation_manager_settings: EscalationManagerSettings,
    pub asserter: ContractAddress,
    pub assertion_time: u64,
    pub settled: bool,
    pub currency: ERC20ABIDispatcher,
    pub expiration_time: u64,
    pub settlement_resolution: bool,
    pub domain_id: u256,
    pub identifier: felt252,
    pub bond: u256,
    pub callback_recipient: ContractAddress,
    pub disputer: ContractAddress,
}

#[derive(starknet::Store, Drop, Serde)]
pub struct WhitelistedCurrency {
    pub is_whitelisted: bool,
    pub final_fee: u256,
}

#[derive(starknet::Store, Drop, Serde)]
pub struct AssertionPolicy {
    pub block_assertion: bool,
    pub arbitrate_via_escalation_manager: bool,
    pub discard_oracle: bool,
    pub validate_disputers: bool
}

#[starknet::interface]
pub trait IOptimisticOracle<TContractState> {
    fn assert_truth_with_defaults(
        ref self: TContractState, claim: ByteArray, asserter: ContractAddress
    ) -> felt252;

    fn assert_truth(
        ref self: TContractState,
        claim: ByteArray,
        asserter: ContractAddress,
        callback_recipient: ContractAddress,
        escalation_manager: ContractAddress,
        liveness: u64,
        currency: ERC20ABIDispatcher,
        bond: u256,
        identifier: felt252,
        domain_id: u256
    ) -> felt252;

    fn dispute_assertion(
        ref self: TContractState, assertion_id: felt252, disputer: ContractAddress
    );

    fn settle_assertion(ref self: TContractState, assertion_id: felt252);
}
#[starknet::interface]
pub trait IFinder<TContractState> {
    fn change_implementation_address(
        ref self: TContractState, interface_name: felt252, implementation_address: ContractAddress
    );

    fn get_implementation_address(
        self: @TContractState, interface_name: felt252
    ) -> ContractAddress;
}

#[starknet::interface]
pub trait IIdentifierWhitelist<TContractState> {
    fn add_supported_identifier(ref self: TContractState, identifier: felt252);

    fn remove_supported_identifier(ref self: TContractState, identifier: felt252);

    fn is_identifier_supported(self: @TContractState, identifier: felt252) -> bool;
}

#[starknet::interface]
pub trait IAddressWhitelist<TContractState> {
    fn add_to_whitelist(ref self: TContractState, new_element: ContractAddress);

    fn remove_from_whitelist(ref self: TContractState, element_to_remove: ContractAddress);

    fn is_on_whitelist(self: @TContractState, element_to_check: ContractAddress) -> bool;

    fn get_whitelist(self: @TContractState) -> Span<ContractAddress>;
}

#[starknet::interface]
pub trait IStore<TContractState> {
    fn pay_oracle_fees(self: @TContractState, erc20_address: ContractAddress, amount: u256);

    fn compute_regular_fee(
        self: @TContractState, start_time: u256, end_time: u256, pfc: u256
    ) -> (u256, u256);

    fn compute_final_fee(self: @TContractState, currency: ContractAddress) -> u256;
}

#[starknet::interface]
pub trait IEscalationManager<TContractState> {
    fn get_assertion_policy(self: @TContractState, assertion_id: felt252) -> AssertionPolicy;

    fn is_dispute_allowed(
        self: @TContractState, assertion_id: felt252, dispute_caller: ContractAddress
    ) -> bool;

    fn get_price(
        self: @TContractState, identifier: felt252, time: u256, ancillary_data: ByteArray
    ) -> u256;

    fn request_price(
        ref self: TContractState, identifier: felt252, time: u256, ancillary_data: ByteArray
    );
}


#[starknet::interface]
pub trait IOracleAncillary<TContractState> {
    fn request_price(
        ref self: TContractState, identifier: felt252, time: u256, ancillary_data: ByteArray
    );

    fn has_price(
        self: @TContractState, identifier: felt252, time: u256, ancillary_data: ByteArray
    ) -> bool;

    fn get_price(
        self: @TContractState, identifier: felt252, time: u256, ancillary_data: ByteArray
    ) -> u256;
}


#[starknet::interface]
pub trait IDisputeCallerConfiguration<TContractState> {
    fn set_dispute_caller_in_whitelist(
        ref self: TContractState, dispute_caller: ContractAddress, value: bool
    );
}

#[starknet::interface]
pub trait IMockOracleAncillaryConfiguration<TContractState> {
    fn get_identifier_whitelist(self: @TContractState) -> IIdentifierWhitelistDispatcher;

    fn push_price(
        ref self: TContractState,
        identifier: felt252,
        time: u256,
        ancillary_data: ByteArray,
        price: u256
    );

    fn push_price_by_request_id(ref self: TContractState, request_id: felt252, price: u256);

    fn get_pending_queries(self: @TContractState) -> Span<QueryPoint>;

    fn get_request_parameters(self: @TContractState, request_id: felt252) -> QueryPoint;
}


#[starknet::interface]
pub trait IOptimisticOracleV3CallbackRecipient<TContractState> {
    fn assertion_resolved_callback(
        self: @TContractState, assertion_id: felt252, asserted_truthfully: bool
    );

    fn assertion_disputed_callback(self: @TContractState, assertion_id: felt252);
}
