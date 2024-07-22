#[starknet::contract]
pub mod optimistic_oracle {
    use core::starknet::event::EventEmitter;
    use optimistic_oracle::contracts::interfaces::{
        IOptimisticOracle, IFinderDispatcher, IFinderDispatcherTrait, WhitelistedCurrency,
        IIdentifierWhitelistDispatcher, IIdentifierWhitelistDispatcherTrait,
        IAddressWhitelistDispatcher, IAddressWhitelistDispatcherTrait, IStoreDispatcher,
        IStoreDispatcherTrait, Assertion, EscalationManagerSettings, AssertionPolicy,
        IEscalationManagerDispatcher, IEscalationManagerDispatcherTrait, IOracleAncillaryDispatcher,
        IOracleAncillaryDispatcherTrait, IOptimisticOracleV3CallbackRecipientDispatcher,
        IOptimisticOracleV3CallbackRecipientDispatcherTrait, IOptimisticOracleV3CallbackRecipient
    };
    use openzeppelin::access::ownable::OwnableComponent;
    use alexandria_data_structures::array_ext::ArrayTraitExt;
    use openzeppelin::token::erc20::interface::{ERC20ABIDispatcher, ERC20ABIDispatcherTrait};
    use alexandria_math::pow;
    use optimistic_oracle::contracts::utils::constants::{OracleInterfaces, PROTOCOL_DECIMALS};
    use optimistic_oracle::contracts::utils::ancillary_data::ancillary_data::{
        append_key_value_address, append_key_value_bytes_32, append_key_value_felt252
    };
    use openzeppelin::security::reentrancyguard::{
        ReentrancyGuardComponent,
        ReentrancyGuardComponent::InternalTrait as InternalReentrancyGuardImpl
    };
    use core::poseidon::poseidon_hash_span;
    use optimistic_oracle::contracts::utils::convert::convert_byte_array_to_felt_array;

    use starknet::{ContractAddress, contract_address_const};
    component!(path: OwnableComponent, storage: ownable, event: OwnableEvent);
    #[abi(embed_v0)]
    impl OwnableImpl = OwnableComponent::OwnableImpl<ContractState>;
    impl OwnableInternalImpl = OwnableComponent::InternalImpl<ContractState>;
    component!(
        path: ReentrancyGuardComponent, storage: reentrancy_guard, event: ReentrancyGuardEvent
    );

    // CONSTANTS DEFINITION
    pub const DEFAULT_IDENTIFIER: felt252 = 'ASSERT_TRUTH';
    pub const NUMERICAL_TRUE: u256 = 1000000000000000000;

    #[storage]
    struct Storage {
        finder: IFinderDispatcher,
        burned_bond_percentage: u256,
        default_liveness: u64,
        default_currency: ERC20ABIDispatcher,
        cached_oracle: ContractAddress, // to replace with actual dispatcher
        cached_currencies: LegacyMap::<ContractAddress, WhitelistedCurrency>,
        cached_identifiers: LegacyMap::<felt252, bool>,
        assertions: LegacyMap::<felt252, Assertion>,
        #[substorage(v0)]
        ownable: OwnableComponent::Storage,
        #[substorage(v0)]
        reentrancy_guard: ReentrancyGuardComponent::Storage,
    }


    mod Errors {
        pub const FINDER_NOT_DEFINED: felt252 = 'Finder not defined';
        pub const BURNED_BOND_PERCENTAGE_ABOVE_100: felt252 = 'Burned bond percentage > 100';
        pub const BURNED_BOND_PERCENTAGE_IS_ZERO: felt252 = 'Burned bond percentage is 0';
        pub const DEFAULT_CURRENCY_IS_ZERO: felt252 = 'Default currency is 0';
        pub const ASSERTER_CANNOT_BE_ZERO: felt252 = 'Asserter cannot be 0';
        pub const ASSERTION_ALREADY_EXISTS: felt252 = 'Assertion already exists';
        pub const UNSUPPORTED_IDENTIFIER: felt252 = 'Unsupported identifier';
        pub const UNSUPPORTED_CURRENCY: felt252 = 'Unsupported currency';
        pub const BOND_AMOUNT_TOO_LOW: felt252 = 'Bond amount too low';
        pub const ASSERTION_NOT_ALLOWED: felt252 = 'Assertion not allowed';
        pub const DISPUTER_CANNOT_BE_ZERO: felt252 = 'Disputer cannot be 0';
        pub const ASSERTION_DOES_NOT_EXIST: felt252 = 'Assertion does not exist';
        pub const ASSERTION_ALREADY_DISPUTED: felt252 = 'Assertion already disputed';
        pub const ASSERTION_IS_EXPIRED: felt252 = 'Assertion is expired';
        pub const DISPUTE_NOT_ALLOWED: felt252 = 'Dispute not allowed';
        pub const ASSERTION_ALREADY_SETTLED: felt252 = 'Assertion already settled';
        pub const ASSERTION_NOT_EXPIRED: felt252 = 'Assertion not expired';
        pub const ASSERTION_NOT_SETTLED: felt252 = 'Assertion not settled';
    }

    #[event]
    #[derive(Drop, starknet::Event)]
    enum Event {
        AdminPropertiesSet: AdminPropertiesSet,
        AssertionSettled: AssertionSettled,
        AssertionMade: AssertionMade,
        #[flat]
        OwnableEvent: OwnableComponent::Event,
        #[flat]
        ReentrancyGuardEvent: ReentrancyGuardComponent::Event,
    }

    #[derive(starknet::Event, Drop)]
    pub struct AdminPropertiesSet {
        pub default_currency: ContractAddress,
        pub default_liveness: u64,
        pub burned_bond_percentage: u256
    }

    #[derive(starknet::Event, Drop)]
    pub struct AssertionSettled {
        pub assertion_id: felt252,
        pub bond_recipient: ContractAddress,
        pub disputed: bool,
        pub settlement_resolution: bool,
        pub settle_caller: ContractAddress,
    }


    #[derive(starknet::Event, Drop)]
    pub struct AssertionMade {
        pub assertion_id: felt252,
        pub domain_id: u256,
        pub claim: ByteArray,
        pub asserter: ContractAddress,
        pub callback_recipient: ContractAddress,
        pub escalation_manager: ContractAddress,
        pub caller: ContractAddress,
        pub expiration_timestamp: u64,
        pub currency: ERC20ABIDispatcher,
        pub bond: u256,
        pub identifier: felt252
    }


    /// Construct the OptimisticOracleV3 contract.
    /// 
    /// # Arguments 
    /// 
    /// * `default_currency` -  the default currency to bond asserters in assert_truth_with_default function. 
    /// * `default_liveness` - the default challenge window time for assertions in assert_truth_with_default function. 
    #[constructor]
    fn constructor(
        ref self: ContractState,
        finder: ContractAddress,
        default_currency: ContractAddress,
        default_liveness: u64,
        owner: ContractAddress
    ) {
        assert(finder != contract_address_const::<0>(), Errors::FINDER_NOT_DEFINED);
        self.finder.write(IFinderDispatcher { contract_address: finder });
        self.set_admin_properties(default_currency, default_liveness, 0);
        self.ownable.initializer(owner);
    }

    #[abi(embed_v0)]
    impl IOptimisticOracleImpl of IOptimisticOracle<ContractState> {
        fn default_identifier(self: @ContractState,) -> felt252 {
            DEFAULT_IDENTIFIER
        }


        fn assert_truth_with_defaults(
            ref self: ContractState, claim: ByteArray, asserter: ContractAddress
        ) -> felt252 {
            let default_liveness = self.default_liveness.read();
            let default_currency = self.default_currency.read();
            self
                .assert_truth(
                    claim,
                    asserter,
                    contract_address_const::<0>(),
                    contract_address_const::<0>(),
                    default_liveness,
                    default_currency,
                    self.get_minimum_bond(default_currency.contract_address),
                    DEFAULT_IDENTIFIER,
                    0
                )
        }

        fn assert_truth(
            ref self: ContractState,
            claim: ByteArray,
            asserter: ContractAddress,
            callback_recipient: ContractAddress,
            escalation_manager: ContractAddress,
            liveness: u64,
            currency: ERC20ABIDispatcher,
            bond: u256,
            identifier: felt252,
            domain_id: u256
        ) -> felt252 {
            self.reentrancy_guard.start();
            let time = starknet::get_block_timestamp();
            let assertion_id = get_id(
                @claim,
                bond,
                time.into(),
                liveness,
                currency,
                callback_recipient,
                escalation_manager,
                identifier
            );
            assert(asserter != contract_address_const::<0>(), Errors::ASSERTER_CANNOT_BE_ZERO);
            let assertion = self.assertions.read(assertion_id).asserter;
            assert(assertion == contract_address_const::<0>(), Errors::ASSERTION_ALREADY_EXISTS);
            assert(self.validate_and_cache_identifier(identifier), Errors::UNSUPPORTED_IDENTIFIER);
            assert(
                self.validate_and_cache_currency(currency.contract_address),
                Errors::UNSUPPORTED_CURRENCY
            );
            assert(
                bond >= self.get_minimum_bond(currency.contract_address),
                Errors::BOND_AMOUNT_TOO_LOW
            );
            let assertion_policy = self.get_assertion_policy(assertion_id);
            assert(!assertion_policy.block_assertion, Errors::ASSERTION_NOT_ALLOWED);
            let caller_address = starknet::get_caller_address();

            let modified_em_settings = EscalationManagerSettings {
                arbitrate_via_escalation_manager: assertion_policy.arbitrate_via_escalation_manager,
                discard_oracle: assertion_policy.discard_oracle,
                validate_disputers: assertion_policy.validate_disputers,
                asserting_caller: starknet::get_caller_address(),
                escalation_manager: escalation_manager
            };
            self
                .assertions
                .write(
                    assertion_id,
                    Assertion {
                        escalation_manager_settings: modified_em_settings,
                        asserter: asserter,
                        disputer: contract_address_const::<0>(),
                        callback_recipient: callback_recipient,
                        currency: currency,
                        domain_id: domain_id,
                        identifier: identifier,
                        bond: bond,
                        settled: false,
                        settlement_resolution: false,
                        assertion_time: time,
                        expiration_time: time + liveness
                    }
                );
            let contract_address = starknet::get_contract_address();
            currency.transfer_from(caller_address, contract_address, bond);
            self
                .emit(
                    AssertionMade {
                        assertion_id: assertion_id,
                        domain_id: domain_id,
                        claim: claim,
                        asserter: asserter,
                        callback_recipient: callback_recipient,
                        escalation_manager: escalation_manager,
                        caller: caller_address,
                        expiration_timestamp: time + liveness,
                        currency: currency,
                        bond: bond,
                        identifier: identifier,
                    }
                );
            self.reentrancy_guard.end();

            assertion_id
        }

        fn dispute_assertion(
            ref self: ContractState, assertion_id: felt252, disputer: ContractAddress
        ) {
            self.reentrancy_guard.start();

            assert(disputer != contract_address_const::<0>(), Errors::DISPUTER_CANNOT_BE_ZERO);
            let assertion = self.assertions.read(assertion_id);
            assert(
                assertion.asserter != contract_address_const::<0>(),
                Errors::ASSERTION_DOES_NOT_EXIST
            );
            assert(
                assertion.disputer == contract_address_const::<0>(),
                Errors::ASSERTION_ALREADY_DISPUTED
            );
            assert(
                assertion.expiration_time > starknet::get_block_timestamp(),
                Errors::ASSERTION_IS_EXPIRED
            );
            assert(self.is_dispute_allowed(assertion_id), Errors::DISPUTE_NOT_ALLOWED);

            assertion
                .currency
                .transfer_from(
                    starknet::get_caller_address(), starknet::get_contract_address(), assertion.bond
                );

            self.reentrancy_guard.end();
        }

        fn settle_assertion(ref self: ContractState, assertion_id: felt252) {
            self.reentrancy_guard.start();
            let mut assertion = self.assertions.read(assertion_id);
            assert(
                assertion.asserter != contract_address_const::<0>(),
                Errors::ASSERTION_DOES_NOT_EXIST
            );
            assert(!assertion.settled, Errors::ASSERTION_ALREADY_SETTLED);
            assertion.settled = true;
            if (assertion.disputer == contract_address_const::<0>()) {
                assert(
                    assertion.expiration_time <= starknet::get_block_timestamp(),
                    Errors::ASSERTION_NOT_EXPIRED
                );
                assertion.settlement_resolution = true;
                assertion.currency.transfer(assertion.asserter, assertion.bond);
                self.callback_on_assertion_resolved(assertion_id, true);
                self.assertions.write(assertion_id, assertion);

                self
                    .emit(
                        AssertionSettled {
                            assertion_id,
                            bond_recipient: assertion.asserter,
                            disputed: false,
                            settlement_resolution: true,
                            settle_caller: starknet::get_caller_address(),
                        }
                    );
            } else {
                let resolved_price = self
                    .oracle_get_price(
                        assertion_id, assertion.identifier, assertion.assertion_time.into()
                    );
                if (assertion.escalation_manager_settings.discard_oracle) {
                    assertion.settlement_resolution = false;
                } else {
                    assertion.settlement_resolution = resolved_price == NUMERICAL_TRUE;
                    let bond_recipient = if (resolved_price == NUMERICAL_TRUE) {
                        assertion.asserter
                    } else {
                        assertion.disputer
                    };
                    let oracle_fee = (self.burned_bond_percentage.read() * assertion.bond)
                        / 1000000000000000000;
                    let bond_recipient_amount = assertion.bond * 2 - oracle_fee;
                    assertion.currency.transfer(self.get_store().contract_address, oracle_fee);
                    assertion.currency.transfer(bond_recipient, bond_recipient_amount);

                    if (!assertion.escalation_manager_settings.discard_oracle) {
                        self
                            .callback_on_assertion_resolved(
                                assertion_id, assertion.settlement_resolution
                            );
                    }

                    self.assertions.write(assertion_id, assertion);

                    self
                        .emit(
                            AssertionSettled {
                                assertion_id,
                                bond_recipient: bond_recipient,
                                disputed: true,
                                settlement_resolution: assertion.settlement_resolution,
                                settle_caller: starknet::get_caller_address(),
                            }
                        )
                }
            }
            self.reentrancy_guard.end();
        }

        fn get_assertion(self: @ContractState, assertion_id: felt252) -> Assertion {
            self.assertions.read(assertion_id)
        }

        fn get_assertion_result(self: @ContractState, assertion_id: felt252) -> bool {
            let assertion = self.assertions.read(assertion_id);
            if (assertion.disputer != contract_address_const::<0>()
                && assertion.escalation_manager_settings.discard_oracle) {
                return false;
            };
            assert(assertion.settled, Errors::ASSERTION_NOT_SETTLED);
            assertion.settlement_resolution
        }

        fn settle_and_get_assertion_result(ref self: ContractState, assertion_id: felt252) -> bool {
            let assertion = self.assertions.read(assertion_id);
            if (!assertion.settled) {
                self.settle_assertion(assertion_id);
            };
            self.get_assertion_result(assertion_id)
        }


        fn sync_params(ref self: ContractState, identifier: felt252, currency: ContractAddress) {
            let cached_oracle = self
                .finder
                .read()
                .get_implementation_address(OracleInterfaces::ORACLE);
            self.cached_oracle.write(cached_oracle);
            self
                .cached_identifiers
                .write(
                    identifier, self.get_identifier_whitelist().is_identifier_supported(identifier)
                );
            let whitelisted_currency = WhitelistedCurrency {
                is_whitelisted: self.get_collateral_whitelist().is_on_whitelist(currency),
                final_fee: self.get_store().compute_final_fee(currency)
            };
            self.cached_currencies.write(currency, whitelisted_currency);
        }


        fn get_minimum_bond(self: @ContractState, currency: ContractAddress) -> u256 {
            let final_fee = self.cached_currencies.read(currency).final_fee;
            let burned_bond_percentage = self.burned_bond_percentage.read();
            (final_fee * pow(10, PROTOCOL_DECIMALS.into())) / burned_bond_percentage
        }

        fn stamp_assertion(self: @ContractState, assertion_id: felt252) -> ByteArray {
            // TODO: write current implementation using Ancillary Implementation
            let mut key: ByteArray = Default::default();
            key.append_word('assertionId', 11);
            let mut current: ByteArray = Default::default();
            let mut oo_key: ByteArray = Default::default();
            oo_key.append_word('ooAsserter', 10);
            let asserter = self.assertions.read(assertion_id).asserter;
            append_key_value_address(
                append_key_value_felt252(current, key, assertion_id), oo_key, asserter
            )
        }
    }

    #[generate_trait]
    impl OOInternalImpl of OOInternalTrait {
        fn set_admin_properties(
            ref self: ContractState,
            default_currency: ContractAddress,
            default_liveness: u64,
            burned_bond_percentage: u256
        ) {
            self.assert_only_owner();
            assert(
                burned_bond_percentage <= pow(10, PROTOCOL_DECIMALS.into()),
                Errors::BURNED_BOND_PERCENTAGE_ABOVE_100
            );
            // assert(burned_bond_percentage > 0 , Errors::BURNED_BOND_PERCENTAGE_IS_ZERO);
            assert(
                default_currency != contract_address_const::<0>(), Errors::DEFAULT_CURRENCY_IS_ZERO
            );
            self.burned_bond_percentage.write(burned_bond_percentage);
            self.default_currency.write(ERC20ABIDispatcher { contract_address: default_currency });
            self.default_liveness.write(default_liveness);
            self.sync_params(DEFAULT_IDENTIFIER, default_currency);
            self
                .emit(
                    AdminPropertiesSet {
                        default_currency, default_liveness, burned_bond_percentage
                    }
                );
        }


        fn get_identifier_whitelist(self: @ContractState,) -> IIdentifierWhitelistDispatcher {
            IIdentifierWhitelistDispatcher {
                contract_address: self
                    .finder
                    .read()
                    .get_implementation_address(OracleInterfaces::IDENTIFIER_WHITELIST)
            }
        }

        fn oracle_get_price(
            self: @ContractState, assertion_id: felt252, identifier: felt252, time: u256
        ) -> u256 {
            self
                .get_oracle(assertion_id)
                .get_price(identifier, time, self.stamp_assertion(assertion_id))
        }

        fn validate_and_cache_identifier(ref self: ContractState, identifier: felt252) -> bool {
            if (self.cached_identifiers.read(identifier)) {
                return true;
            }
            self
                .cached_identifiers
                .write(
                    identifier, self.get_identifier_whitelist().is_identifier_supported(identifier)
                );
            self.cached_identifiers.read(identifier)
        }

        fn validate_and_cache_currency(ref self: ContractState, currency: ContractAddress) -> bool {
            if (self.cached_currencies.read(currency).is_whitelisted) {
                return true;
            }
            let is_whitelisted = self.get_collateral_whitelist().is_on_whitelist(currency);
            let final_fee = self.get_store().compute_final_fee(currency);
            let cached_currency = WhitelistedCurrency { is_whitelisted, final_fee: final_fee };
            self.cached_currencies.write(currency, cached_currency);
            is_whitelisted
        }
        fn get_collateral_whitelist(self: @ContractState) -> IAddressWhitelistDispatcher {
            IAddressWhitelistDispatcher {
                contract_address: self
                    .finder
                    .read()
                    .get_implementation_address(OracleInterfaces::COLLATERAL_WHITELIST)
            }
        }

        fn is_dispute_allowed(self: @ContractState, assertion_id: felt252) -> bool {
            if (!self
                .assertions
                .read(assertion_id)
                .escalation_manager_settings
                .validate_disputers) {
                return true;
            }
            let em = self
                .assertions
                .read(assertion_id)
                .escalation_manager_settings
                .escalation_manager;
            if (em == contract_address_const::<0>()) {
                return true;
            };
            return IEscalationManagerDispatcher { contract_address: em }
                .is_dispute_allowed(assertion_id, starknet::get_caller_address());
        }

        fn get_assertion_policy(self: @ContractState, assertion_id: felt252) -> AssertionPolicy {
            let em = self.get_escalation_manager(assertion_id);
            if (em == contract_address_const::<0>()) {
                return AssertionPolicy {
                    block_assertion: false,
                    arbitrate_via_escalation_manager: false,
                    discard_oracle: false,
                    validate_disputers: false
                };
            }
            IEscalationManagerDispatcher { contract_address: em }.get_assertion_policy(assertion_id)
        }

        fn get_escalation_manager(self: @ContractState, assertion_id: felt252) -> ContractAddress {
            self.assertions.read(assertion_id).escalation_manager_settings.escalation_manager
        }

        fn oracle_request_price(
            self: @ContractState, assertion_id: felt252, identifier: felt252, time: u256
        ) {
            self
                .get_oracle(assertion_id)
                .request_price(identifier, time, self.stamp_assertion(assertion_id));
        }

        fn get_oracle(self: @ContractState, assertion_id: felt252) -> IOracleAncillaryDispatcher {
            if (self
                .assertions
                .read(assertion_id)
                .escalation_manager_settings
                .arbitrate_via_escalation_manager) {
                return IOracleAncillaryDispatcher {
                    contract_address: self.get_escalation_manager(assertion_id)
                };
            }
            IOracleAncillaryDispatcher { contract_address: self.cached_oracle.read() }
        }

        fn get_store(self: @ContractState) -> IStoreDispatcher {
            IStoreDispatcher {
                contract_address: self
                    .finder
                    .read()
                    .get_implementation_address(OracleInterfaces::STORE)
            }
        }


        fn assert_only_owner(self: @ContractState) {
            self.ownable.assert_only_owner();
        }


        fn callback_on_assertion_resolved(
            ref self: ContractState, assertion_id: felt252, asserted_truthfully: bool
        ) {
            let cr = self.assertions.read(assertion_id).callback_recipient;
            let em = self.get_escalation_manager(assertion_id);
            if (cr != contract_address_const::<0>()) {
                IOptimisticOracleV3CallbackRecipientDispatcher { contract_address: cr }
                    .assertion_resolved_callback(assertion_id, asserted_truthfully);
            }
            if (em != contract_address_const::<0>()) {
                IOptimisticOracleV3CallbackRecipientDispatcher { contract_address: em }
                    .assertion_resolved_callback(assertion_id, asserted_truthfully);
            }
        }
    }


    fn get_id(
        claim: @ByteArray,
        bond: u256,
        time: u256,
        liveness: u64,
        currency: ERC20ABIDispatcher,
        callback_recipient: ContractAddress,
        escalation_manager: ContractAddress,
        identifier: felt252
    ) -> felt252 {
        let mut converted_claim: Array<felt252> = convert_byte_array_to_felt_array(claim);
        converted_claim
            .concat(
                @array![
                    bond.high.into(),
                    bond.low.into(),
                    time.high.into(),
                    time.low.into(),
                    liveness.into(),
                    currency.contract_address.into(),
                    callback_recipient.into(),
                    escalation_manager.into(),
                    identifier
                ]
            );
        poseidon_hash_span(converted_claim.span())
    }
}
