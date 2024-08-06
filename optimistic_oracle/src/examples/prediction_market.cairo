#[starknet::contract]
pub mod prediction_market {
    use starknet::{ContractAddress, contract_address_const, ClassHash, syscalls::deploy_syscall};
    use openzeppelin::token::erc20::interface::{ERC20ABIDispatcher, ERC20ABIDispatcherTrait};
    use optimistic_oracle::contracts::interfaces::{
        IFinderDispatcher, IFinderDispatcherTrait, IOptimisticOracleDispatcherTrait,
        IOptimisticOracleDispatcher, IOptimisticOracleCallbackRecipient, IPredictionMarket,
        IExtendedERC20Dispatcher, IExtendedERC20DispatcherTrait, IAddressWhitelistDispatcher,
        IAddressWhitelistDispatcherTrait
    };
    use optimistic_oracle::contracts::common::address_whitelist::address_whitelist::WhitelistType;
    use optimistic_oracle::contracts::utils::keccak::compute_keccak_byte_array;
    use optimistic_oracle::contracts::mocks::full_erc20::full_erc20::{MINTER_ROLE, BURNER_ROLE};
    use optimistic_oracle::contracts::optimistic_oracle_v1::optimistic_oracle_v1::DEFAULT_IDENTIFIER;
    use core::poseidon::poseidon_hash_span;
    use optimistic_oracle::contracts::utils::convert::{byte_array_as_felt_array};
    use alexandria_data_structures::array_ext::ArrayTraitExt;
    use optimistic_oracle::contracts::utils::constants::OracleInterfaces;


    // CONSTANTS DEFINITION
    pub const ASSERTION_LIVENESS: u64 = 7200;


    pub mod Errors {
        pub const UNSUPPORTED_CURRENCY: felt252 = 'Unsupported currency';
        pub const NOT_AUTHORIZED: felt252 = 'Not authorized';
        pub const EMPTY_FIRST_OUTCOME: felt252 = 'Empty first outcome';
        pub const EMPTY_SECOND_OUTCOME: felt252 = 'Empty second outcome';
        pub const IDENTICAL_OUTCOME: felt252 = 'Identical outcome';
        pub const EMPTY_DESCRIPTION: felt252 = 'Empty description';
        pub const MARKET_ALREADY_EXISTS: felt252 = 'Market already exists';
        pub const MARKET_DOES_NOT_EXIST: felt252 = 'Market does not exist';
        pub const ASSERTION_ACTIVE_OR_RESOLVED: felt252 = 'Assertion active or resolved';
        pub const INVALID_ASSERTED_OUTCOME: felt252 = 'Invalid asserted outcome';
        pub const MARKET_NOT_RESOLVED: felt252 = 'Market is not resolved';
    }

    #[derive(Drop, starknet::Store, Serde, Clone)]
    pub struct Market {
        resolved: bool,
        asserted_outcome_id: u256,
        outcome1_token: IExtendedERC20Dispatcher,
        outcome2_token: IExtendedERC20Dispatcher,
        reward: u256,
        required_bond: u256,
        outcome1: ByteArray,
        outcome2: ByteArray,
        description: ByteArray,
    }

    #[derive(Drop, starknet::Store, Serde)]
    struct AssertedMarket {
        asserter: ContractAddress,
        market_id: felt252
    }


    #[derive(starknet::Event, Drop)]
    pub struct MarketInitialized {
        market_id: felt252,
        outcome1: ByteArray,
        outcome2: ByteArray,
        description: ByteArray,
        outcome1_token: ContractAddress,
        outcome2_token: ContractAddress,
        reward: u256,
        required_bond: u256,
    }

    #[derive(starknet::Event, Drop)]
    pub struct MarketAsserted {
        market_id: felt252,
        asserted_outcome: ByteArray,
        assertion_id: felt252
    }

    #[derive(starknet::Event, Drop)]
    pub struct MarketResolved {
        market_id: felt252,
    }

    #[derive(starknet::Event, Drop)]
    pub struct TokensCreated {
        market_id: felt252,
        account: ContractAddress,
        tokens_created: u256,
    }

    #[derive(starknet::Event, Drop)]
    pub struct TokensRedeemed {
        market_id: felt252,
        account: ContractAddress,
        tokens_redeemed: u256,
    }

    #[derive(starknet::Event, Drop)]
    pub struct TokensSettled {
        market_id: felt252,
        account: ContractAddress,
        payout: u256,
        outcome1_tokens: u256,
        outcome2_tokens: u256,
    }


    #[event]
    #[derive(Drop, starknet::Event)]
    enum Event {
        MarketInitialized: MarketInitialized,
        MarketAsserted: MarketAsserted,
        MarketResolved: MarketResolved,
        TokensCreated: TokensCreated,
        TokensRedeemed: TokensRedeemed,
        TokensSettled: TokensSettled,
    }


    #[storage]
    struct Storage {
        markets: LegacyMap::<felt252, Market>,
        erc20_class_hash: ClassHash,
        asserted_markets: LegacyMap::<felt252, AssertedMarket>,
        finder: IFinderDispatcher,
        currency: ERC20ABIDispatcher,
        oo: IOptimisticOracleDispatcher,
        default_identifier: felt252,
    }


    #[constructor]
    fn constructor(
        ref self: ContractState,
        finder: ContractAddress,
        currency: ContractAddress,
        optimistic_oracle: ContractAddress,
        erc20_class_hash: ClassHash
    ) {
        self.finder.write(IFinderDispatcher { contract_address: finder });
        assert(
            self.get_collateral_whitelist().is_on_whitelist(currency, WhitelistType::Currency),
            Errors::UNSUPPORTED_CURRENCY
        );
        self.currency.write(ERC20ABIDispatcher { contract_address: currency });
        self.oo.write(IOptimisticOracleDispatcher { contract_address: optimistic_oracle });
        self.default_identifier.write(DEFAULT_IDENTIFIER);
        self.erc20_class_hash.write(erc20_class_hash)
    }

    #[abi(embed_v0)]
    impl IOptimisticOracleCallbackRecipientImpl of IOptimisticOracleCallbackRecipient<
        ContractState
    > {
        fn assertion_resolved_callback(
            ref self: ContractState, assertion_id: felt252, asserted_truthfully: bool
        ) {
            assert(
                starknet::get_caller_address() == self.oo.read().contract_address,
                Errors::NOT_AUTHORIZED
            );
            let mut market: Market = self
                .markets
                .read(self.asserted_markets.read(assertion_id).market_id);

            if (asserted_truthfully) {
                market.resolved = true;
                if (market.reward > 0) {
                    ERC20ABIDispatcher { contract_address: self.currency.read().contract_address }
                        .transfer(self.asserted_markets.read(assertion_id).asserter, market.reward);
                    self
                        .emit(
                            MarketResolved {
                                market_id: self.asserted_markets.read(assertion_id).market_id
                            }
                        );
                } else {
                    market.asserted_outcome_id = 0;
                }
            }
            self.markets.write(self.asserted_markets.read(assertion_id).market_id, market);
        }
        fn assertion_disputed_callback(self: @ContractState, assertion_id: felt252) {}
    }


    #[abi(embed_v0)]
    impl IPredictionMarketImpl of IPredictionMarket<ContractState> {
        fn get_market(self: @ContractState, market_id: felt252) -> Market {
            self.markets.read(market_id)
        }

        fn initialize_market(
            ref self: ContractState,
            outcome1: ByteArray,
            outcome2: ByteArray,
            description: ByteArray,
            reward: u256,
            required_bond: u256
        ) -> felt252 {
            assert(outcome1.len() > 0, Errors::EMPTY_FIRST_OUTCOME);
            assert(outcome2.len() > 0, Errors::EMPTY_SECOND_OUTCOME);
            assert(outcome1 != outcome2, Errors::IDENTICAL_OUTCOME);
            assert(description.len() > 0, Errors::EMPTY_DESCRIPTION);
            let input: Array<felt252> = array![starknet::get_block_number().into()]
                .concat(@byte_array_as_felt_array(@description));
            let market_id = poseidon_hash_span(input.span());
            assert(
                self
                    .markets
                    .read(market_id)
                    .outcome1_token
                    .contract_address == contract_address_const::<0>(),
                Errors::MARKET_ALREADY_EXISTS
            );
            let mut extension: ByteArray = Default::default();
            extension.append_word(' Token', 6);

            let token1_name = ByteArrayTrait::concat(@outcome1, @extension);
            let token1_symbol: ByteArray = "01T";
            let token1_salt = poseidon_hash_span(
                array![starknet::get_caller_address().into()]
                    .concat(
                        @byte_array_as_felt_array(
                            @ByteArrayTrait::concat(@token1_name, @description)
                        )
                    )
                    .span()
            );
            let token2_name = ByteArrayTrait::concat(@outcome2, @extension);
            let token2_symbol: ByteArray = "02T";
            let token2_salt = poseidon_hash_span(
                array![starknet::get_caller_address().into()]
                    .concat(
                        @byte_array_as_felt_array(
                            @ByteArrayTrait::concat(@token2_name, @description)
                        )
                    )
                    .span()
            );
            let mut token1_initializer = array![];
            token1_name.serialize(ref token1_initializer);
            token1_symbol.serialize(ref token1_initializer);
            let mut token2_initializer = array![];
            token2_name.serialize(ref token2_initializer);
            token2_symbol.serialize(ref token2_initializer);
            // erc20 contract class must pass the name / symbol as const
            let (token1_address, _,) = deploy_syscall(
                self.erc20_class_hash.read(), token1_salt, token1_initializer.span(), true
            )
                .unwrap();
            let (token2_address, _,) = deploy_syscall(
                self.erc20_class_hash.read(), token2_salt, token2_initializer.span(), true
            )
                .unwrap();
            let token1_dispatcher = IExtendedERC20Dispatcher { contract_address: token1_address };
            token1_dispatcher.grant_minter_role(starknet::get_contract_address());
            token1_dispatcher.grant_burner_role(starknet::get_contract_address());
            let token2_dispatcher = IExtendedERC20Dispatcher { contract_address: token2_address };
            token2_dispatcher.grant_minter_role(starknet::get_contract_address());
            token2_dispatcher.grant_burner_role(starknet::get_contract_address());
            self
                .markets
                .write(
                    market_id,
                    Market {
                        resolved: false,
                        asserted_outcome_id: 0,
                        outcome1_token: IExtendedERC20Dispatcher {
                            contract_address: token1_address
                        },
                        outcome2_token: IExtendedERC20Dispatcher {
                            contract_address: token2_address
                        },
                        reward: reward,
                        required_bond: required_bond,
                        outcome1: outcome1.clone(),
                        outcome2: outcome2.clone(),
                        description: description.clone()
                    }
                );
            if (reward > 0) {
                self
                    .currency
                    .read()
                    .transfer_from(
                        starknet::get_caller_address(), starknet::get_contract_address(), reward
                    );
            }
            self
                .emit(
                    MarketInitialized {
                        market_id,
                        outcome1,
                        outcome2,
                        description,
                        outcome1_token: token1_address,
                        outcome2_token: token2_address,
                        reward,
                        required_bond
                    }
                );
            market_id
        }


        fn assert_market(
            ref self: ContractState, market_id: felt252, asserted_outcome: ByteArray
        ) -> felt252 {
            let UNRESOLVABLE: ByteArray = "Unresolvable";
            let mut market = self.markets.read(market_id);
            assert(
                market.outcome1_token.contract_address != contract_address_const::<0>(),
                Errors::MARKET_DOES_NOT_EXIST
            );
            let asserted_outcome_id = compute_keccak_byte_array(@asserted_outcome);
            assert(market.asserted_outcome_id == 0, Errors::ASSERTION_ACTIVE_OR_RESOLVED);
            assert(
                asserted_outcome_id == compute_keccak_byte_array(@market.outcome1)
                    || asserted_outcome_id == compute_keccak_byte_array(@market.outcome2)
                    || asserted_outcome_id == compute_keccak_byte_array(@UNRESOLVABLE),
                Errors::INVALID_ASSERTED_OUTCOME
            );

            market.asserted_outcome_id = asserted_outcome_id;
            self.markets.write(market_id, market.clone());
            let minimum_bond = self
                .oo
                .read()
                .get_minimum_bond(self.currency.read().contract_address);
            let bond = if (market.required_bond > minimum_bond) {
                market.required_bond
            } else {
                minimum_bond
            };

            let claim = compose_claim(asserted_outcome.clone(), market.description);
            self
                .currency
                .read()
                .transfer_from(
                    starknet::get_caller_address(), starknet::get_contract_address(), bond
                );
            self.currency.read().approve(self.oo.read().contract_address, bond);
            let assertion_id = self.assert_thruth_with_defaults(claim, bond);

            self
                .asserted_markets
                .write(
                    assertion_id,
                    AssertedMarket {
                        asserter: starknet::get_caller_address(), market_id: market_id
                    }
                );
            self.emit(MarketAsserted { market_id, asserted_outcome, assertion_id });
            assertion_id
        }

        fn create_outcome_tokens(
            ref self: ContractState, market_id: felt252, tokens_to_create: u256
        ) {
            let market = self.markets.read(market_id);
            assert(
                market.outcome1_token.contract_address != contract_address_const::<0>(),
                Errors::MARKET_DOES_NOT_EXIST
            );
            self
                .currency
                .read()
                .transfer_from(
                    starknet::get_caller_address(),
                    starknet::get_contract_address(),
                    tokens_to_create
                );
            market.outcome1_token.mint(starknet::get_caller_address(), tokens_to_create);
            market.outcome2_token.mint(starknet::get_caller_address(), tokens_to_create);
            self
                .emit(
                    TokensCreated {
                        market_id,
                        account: starknet::get_caller_address(),
                        tokens_created: tokens_to_create
                    }
                );
        }


        fn redeem_outcome_tokens(
            ref self: ContractState, market_id: felt252, tokens_to_redeeem: u256
        ) {
            let market = self.markets.read(market_id);
            assert(
                market.outcome1_token.contract_address != contract_address_const::<0>(),
                Errors::MARKET_DOES_NOT_EXIST
            );
            self.currency.read().transfer(starknet::get_caller_address(), tokens_to_redeeem);
            market.outcome1_token.burn(starknet::get_caller_address(), tokens_to_redeeem);
            market.outcome2_token.burn(starknet::get_caller_address(), tokens_to_redeeem);
            self
                .emit(
                    TokensRedeemed {
                        market_id,
                        account: starknet::get_caller_address(),
                        tokens_redeemed: tokens_to_redeeem
                    }
                );
        }

        fn settle_outcome_tokens(ref self: ContractState, market_id: felt252) -> u256 {
            let caller = starknet::get_caller_address();
            let market = self.markets.read(market_id);
            assert(market.resolved, Errors::MARKET_NOT_RESOLVED);
            let outcome1_balance = ERC20ABIDispatcher {
                contract_address: market.outcome1_token.contract_address
            }
                .balance_of(starknet::get_caller_address());
            let outcome2_balance = ERC20ABIDispatcher {
                contract_address: market.outcome2_token.contract_address
            }
                .balance_of(starknet::get_caller_address());

            let payout = if (market
                .asserted_outcome_id == compute_keccak_byte_array(@market.outcome1)) {
                outcome1_balance
            } else if (market.asserted_outcome_id == compute_keccak_byte_array(@market.outcome2)) {
                outcome2_balance
            } else {
                (outcome1_balance + outcome2_balance) / 2
            };

            market.outcome1_token.burn(caller, outcome1_balance);
            market.outcome2_token.burn(caller, outcome2_balance);
            self.currency.read().transfer(caller, payout);

            self
                .emit(
                    TokensSettled {
                        market_id,
                        account: caller,
                        payout,
                        outcome1_tokens: outcome1_balance,
                        outcome2_tokens: outcome2_balance,
                    }
                );
            payout
        }
    }

    #[generate_trait]
    impl InternalTraitImpl of InternalTrait {
        fn assert_thruth_with_defaults(
            self: @ContractState, claim: ByteArray, bond: u256
        ) -> felt252 {
            self
                .oo
                .read()
                .assert_truth(
                    claim,
                    starknet::get_caller_address(),
                    starknet::get_contract_address(),
                    contract_address_const::<0>(),
                    ASSERTION_LIVENESS,
                    self.currency.read(),
                    bond,
                    self.default_identifier.read(),
                    0
                )
        }

        fn get_collateral_whitelist(self: @ContractState) -> IAddressWhitelistDispatcher {
            IAddressWhitelistDispatcher {
                contract_address: self
                    .finder
                    .read()
                    .get_implementation_address(OracleInterfaces::COLLATERAL_WHITELIST)
            }
        }
    }

    fn compose_claim(outcome: ByteArray, description: ByteArray) -> ByteArray {
        let mut claim: ByteArray = Default::default();
        let p1: ByteArray = "As of assertion timestamp ";
        let p2: ByteArray = ", the described prediction market outcome is: ";
        let p3: ByteArray = ". The market description is: ";
        let mut block_timestamp: ByteArray = Default::default();
        block_timestamp.append_word(starknet::get_block_timestamp().into(), 8);
        claim = ByteArrayTrait::concat(@claim, @p1);
        claim = ByteArrayTrait::concat(@claim, @block_timestamp);
        claim = ByteArrayTrait::concat(@claim, @p2);
        claim = ByteArrayTrait::concat(@claim, @outcome);
        claim = ByteArrayTrait::concat(@claim, @p3);
        claim = ByteArrayTrait::concat(@claim, @description);
        claim
    }
}
