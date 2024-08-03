#[starknet::contract]
pub mod mock_oracle_ancillary {
    use starknet::{ContractAddress, ClassHash};
    use optimistic_oracle::contracts::interfaces::{
        IFinderDispatcher, IFinderDispatcherTrait, IOracleAncillary,
        IMockOracleAncillaryConfiguration, IIdentifierWhitelistDispatcher,
        IIdentifierWhitelistDispatcherTrait
    };
    use openzeppelin::access::ownable::OwnableComponent;
    use optimistic_oracle::contracts::utils::constants::OracleInterfaces;
    use optimistic_oracle::contracts::utils::convert::convert_byte_array_to_felt_array;
    use core::poseidon::poseidon_hash_span;
    use openzeppelin::upgrades::{interface::IUpgradeable, upgradeable::UpgradeableComponent};
    component!(path: OwnableComponent, storage: ownable, event: OwnableEvent);
    component!(path: UpgradeableComponent, storage: upgradeable, event: UpgradeableEvent);

    #[abi(embed_v0)]
    impl OwnableImpl = OwnableComponent::OwnableImpl<ContractState>;
    impl OwnableInternalImpl = OwnableComponent::InternalImpl<ContractState>;
    impl UpgradeableInternalImpl = UpgradeableComponent::InternalImpl<ContractState>;


    #[derive(starknet::Store, Drop)]
    pub struct Price {
        is_available: bool,
        price: u256,
        verified_time: u256
    }

    #[derive(starknet::Store, Drop)]
    pub struct QueryIndex {
        is_valid: bool,
        index: Option::<u128>,
    }

    #[derive(starknet::Store, Drop, Serde)]
    pub struct QueryPoint {
        pub identifier: felt252,
        pub time: u256,
        pub ancillary_data: ByteArray
    }

    pub mod Errors {
        pub const IDENTIFIER_NOT_SUPPORTED: felt252 = 'Identifier not supported';
        pub const PRICE_NOT_REQUESTED: felt252 = 'Price not requested';
        pub const REQUEST_ID_NOT_FOUND: felt252 = 'Request id not found';
        pub const PRICE_NOT_AVAILABLE: felt252 = 'Price not available';
        pub const REQUEST_DELETED: felt252 = 'Request deleted';
    }

    #[derive(starknet::Event, Drop)]
    pub struct PriceRequestAdded {
        pub requester: ContractAddress,
        pub identifier: felt252,
        pub time: u256,
        pub ancillary_data: ByteArray,
        pub request_id: felt252,
    }

    #[derive(starknet::Event, Drop)]
    pub struct PushedPrice {
        pub pusher: ContractAddress,
        pub identifier: felt252,
        pub time: u256,
        pub ancillary_data: ByteArray,
        pub price: u256,
        pub request_id: felt252,
    }
    #[event]
    #[derive(Drop, starknet::Event)]
    pub enum Event {
        PriceRequestAdded: PriceRequestAdded,
        PushedPrice: PushedPrice,
        #[flat]
        UpgradeableEvent: UpgradeableComponent::Event,
        #[flat]
        OwnableEvent: OwnableComponent::Event,
    }

    #[storage]
    struct Storage {
        finder: IFinderDispatcher,
        verified_prices: LegacyMap::<felt252, Price>,
        query_indices: LegacyMap::<felt252, QueryIndex>,
        requested_prices_len: u128,
        requested_prices: LegacyMap::<u128, QueryPoint>,
        #[substorage(v0)]
        ownable: OwnableComponent::Storage,
        #[substorage(v0)]
        upgradeable: UpgradeableComponent::Storage,
    }


    #[constructor]
    fn constructor(ref self: ContractState, finder: ContractAddress, owner: ContractAddress) {
        self.ownable.initializer(owner);
        self.finder.write(IFinderDispatcher { contract_address: finder });
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
    impl IMockOracleAncillaryImpl of IOracleAncillary<ContractState> {
        fn request_price(
            ref self: ContractState, identifier: felt252, time: u256, ancillary_data: ByteArray
        ) {
            assert(
                self.get_identifier_whitelist().is_identifier_supported(identifier),
                Errors::IDENTIFIER_NOT_SUPPORTED
            );
            let request_id = encode_price_request(identifier, time, @ancillary_data);
            let lookup = self.verified_prices.read(request_id);
            if (!lookup.is_available && !self.query_indices.read(request_id).is_valid) {
                let requested_price_index = self.requested_prices_len.read();
                self
                    .query_indices
                    .write(
                        request_id,
                        QueryIndex { is_valid: true, index: Option::Some(requested_price_index) }
                    );
                let cloned_ancillary_data = ancillary_data.clone();
                self
                    .requested_prices
                    .write(
                        requested_price_index,
                        QueryPoint { identifier, time, ancillary_data: cloned_ancillary_data }
                    );
                self.requested_prices_len.write(requested_price_index + 1);
                self
                    .emit(
                        PriceRequestAdded {
                            requester: starknet::get_caller_address(),
                            identifier: identifier,
                            time: time,
                            ancillary_data: ancillary_data,
                            request_id: request_id,
                        }
                    );
            }
        }

        fn has_price(
            self: @ContractState, identifier: felt252, time: u256, ancillary_data: ByteArray
        ) -> bool {
            let lookup = self
                .verified_prices
                .read(encode_price_request(identifier, time, @ancillary_data));
            lookup.is_available
        }

        fn get_price(
            self: @ContractState, identifier: felt252, time: u256, ancillary_data: ByteArray
        ) -> u256 {
            let lookup = self
                .verified_prices
                .read(encode_price_request(identifier, time, @ancillary_data));
            assert(lookup.is_available, Errors::PRICE_NOT_AVAILABLE);
            lookup.price
        }
    }

    #[abi(embed_v0)]
    impl IMockOracleAncillaryConfigurationImpl of IMockOracleAncillaryConfiguration<ContractState> {
        fn get_identifier_whitelist(self: @ContractState) -> IIdentifierWhitelistDispatcher {
            IIdentifierWhitelistDispatcher {
                contract_address: self
                    .finder
                    .read()
                    .get_implementation_address(OracleInterfaces::IDENTIFIER_WHITELIST)
            }
        }

        fn push_price(
            ref self: ContractState,
            identifier: felt252,
            time: u256,
            ancillary_data: ByteArray,
            price: u256
        ) {
            let request_id = encode_price_request(identifier, time, @ancillary_data);
            self
                .verified_prices
                .write(
                    request_id,
                    Price {
                        is_available: true,
                        price,
                        verified_time: starknet::get_block_timestamp().into()
                    }
                );
            let query_index = self.query_indices.read(request_id);
            assert(query_index.is_valid, Errors::PRICE_NOT_REQUESTED);

            let index_to_replace = match query_index.index {
                Option::Some(index) => { index },
                Option::None => {
                    panic(array![Errors::PRICE_NOT_REQUESTED]);
                    0
                }
            };
            self
                .query_indices
                .write(request_id, QueryIndex { is_valid: false, index: Option::None });
            self.requested_prices_len.write(self.requested_prices_len.read() - 1);
            let last_index = self.requested_prices_len.read();
            if last_index != index_to_replace {
                let query_to_copy = self.requested_prices.read(last_index);
                let id = encode_price_request(
                    query_to_copy.identifier, query_to_copy.time, @query_to_copy.ancillary_data
                );
                let mut query = self.query_indices.read(id);
                query.index = Option::Some(index_to_replace);
                self.query_indices.write(id, query);
                self.requested_prices.write(index_to_replace, query_to_copy);
            }
            self
                .emit(
                    PushedPrice {
                        pusher: starknet::get_caller_address(),
                        identifier: identifier,
                        time: time,
                        ancillary_data: ancillary_data,
                        price: price,
                        request_id: request_id,
                    }
                );
        }

        fn push_price_by_request_id(ref self: ContractState, request_id: felt252, price: u256) {
            let query_point = self.get_request_parameters(request_id);
            self
                .push_price(
                    query_point.identifier, query_point.time, query_point.ancillary_data, price
                );
        }

        fn get_pending_queries(self: @ContractState) -> Span<QueryPoint> {
            let mut queries: Array<QueryPoint> = array![];
            let requested_prices_len = self.requested_prices_len.read();
            let mut cur_idx = 0;
            loop {
                if (cur_idx == requested_prices_len) {
                    break;
                }
                queries.append(self.requested_prices.read(cur_idx));
                cur_idx += 1;
            };
            queries.span()
        }

        fn get_request_parameters(self: @ContractState, request_id: felt252) -> QueryPoint {
            let query_index = self.query_indices.read(request_id);
            assert(query_index.is_valid, Errors::REQUEST_ID_NOT_FOUND);
            match query_index.index {
                Option::Some(index) => self.requested_prices.read(index),
                Option::None => {
                    panic(array![Errors::REQUEST_DELETED]);
                    QueryPoint { identifier: 0, time: 0, ancillary_data: Default::default() }
                }
            }
        }
    }


    fn encode_price_request(
        identifier: felt252, time: u256, ancillary_data: @ByteArray
    ) -> felt252 {
        let mut input: Array<felt252> = array![identifier, time.high.into(), time.low.into()];
        let mut ancillary_data_felt: Array<felt252> = convert_byte_array_to_felt_array(
            ancillary_data
        );
        let mut cur_idx = 0;
        loop {
            if (cur_idx == ancillary_data_felt.len()) {
                break;
            }
            input.append(*ancillary_data_felt.at(cur_idx));
            cur_idx += 1;
        };
        poseidon_hash_span(input.span())
    }
}
