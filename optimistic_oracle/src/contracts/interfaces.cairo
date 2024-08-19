use core::byte_array::{ByteArray, ByteArrayTrait};
use starknet::ContractAddress;
use openzeppelin::token::erc20::interface::ERC20ABIDispatcher;
use optimistic_oracle::contracts::mocks::oracle_ancillary::mock_oracle_ancillary::{
    QueryPoint, QueryIndex
};
use optimistic_oracle::examples::prediction_market::prediction_market::Market;

#[derive(starknet::Store, Drop, Serde, Copy)]
pub struct EscalationManagerSettings {
    pub arbitrate_via_escalation_manager: bool,
    pub discard_oracle: bool,
    pub validate_disputers: bool,
    pub asserting_caller: ContractAddress,
    pub escalation_manager: ContractAddress,
}

#[derive(starknet::Store, Drop, Serde, Copy)]
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

    fn get_minimum_bond(self: @TContractState, currency: ContractAddress) -> u256;

    fn stamp_assertion(self: @TContractState, assertion_id: felt252) -> ByteArray;

    fn default_identifier(self: @TContractState,) -> felt252;

    fn get_assertion(self: @TContractState, assertion_id: felt252) -> Assertion;

    fn sync_params(ref self: TContractState, identifier: felt252, currency: ContractAddress);

    fn settle_and_get_assertion_result(ref self: TContractState, assertion_id: felt252) -> bool;

    fn get_assertion_result(self: @TContractState, assertion_id: felt252) -> bool;

    fn set_admin_properties(
        ref self: TContractState,
        default_currency: ContractAddress,
        default_liveness: u64,
        burned_bond_percentage: u256
    );
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

    fn compute_final_fee(self: @TContractState, currency: ContractAddress) -> u256;

    fn set_final_fee(ref self: TContractState, currency: ContractAddress, new_final_fee: u256);

    fn withdraw_funds(ref self: TContractState, receiver: ContractAddress);
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
    ) -> felt252;
}


#[starknet::interface]
pub trait IOracleAncillary<TContractState> {
    fn request_price(
        ref self: TContractState, identifier: felt252, time: u256, ancillary_data: ByteArray
    ) -> felt252;

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
        ref self: TContractState, assertion_id: felt252, asserted_truthfully: bool
    );

    fn assertion_disputed_callback(self: @TContractState, assertion_id: felt252);
}


#[starknet::interface]
pub trait IPredictionMarket<TContractState> {
    fn get_market(self: @TContractState, market_id: felt252) -> Market;

    fn initialize_market(
        ref self: TContractState,
        outcome1: ByteArray,
        outcome2: ByteArray,
        description: ByteArray,
        reward: u256,
        required_bond: u256
    ) -> felt252;

    fn create_outcome_tokens(ref self: TContractState, market_id: felt252, tokens_to_create: u256);

    fn redeem_outcome_tokens(ref self: TContractState, market_id: felt252, tokens_to_redeeem: u256);

    fn assert_market(
        ref self: TContractState, market_id: felt252, asserted_outcome: ByteArray
    ) -> felt252;

    fn settle_outcome_tokens(ref self: TContractState, market_id: felt252) -> u256;
}


#[starknet::interface]
pub trait IExtendedERC20<TContractState> {
    fn mint(ref self: TContractState, recipient: ContractAddress, value: u256);

    fn burn(ref self: TContractState, account: ContractAddress, value: u256);


    fn only_owner(self: @TContractState, caller_address: ContractAddress) -> bool;

    fn grant_minter_role(ref self: TContractState, minter: ContractAddress);

    fn revoke_minter_role(ref self: TContractState, account: ContractAddress);


    fn grant_burner_role(ref self: TContractState, burner: ContractAddress);

    fn revoke_burner_role(ref self: TContractState, account: ContractAddress);
}
