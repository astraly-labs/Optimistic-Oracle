#[starknet::contract]
pub mod mock_oracle_ancillary {
    use starknet::ContractAddress;
    use optimistic_oracle::contracts::interfaces::{
        IFinderDispatcher, IFinderDispatcherTrait, IOracleAncillary,
        IMockOracleAncillaryConfiguration, IIdentifierWhitelistDispatcher,
        IIdentifierWhitelistDispatcherTrait
    };
    use optimistic_oracle::contracts::utils::constants::OracleInterfaces;
    use optimistic_oracle::contracts::utils::convert::convert_byte_array_to_felt_array;
    use alexandria_data_structures::array_ext::ArrayTraitExt;
    use core::poseidon::poseidon_hash_span;

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
        identifier: felt252,
        time: u256,
        ancillary_data: ByteArray
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
    enum Event {
        PriceRequestAdded: PriceRequestAdded,
        PushedPrice: PushedPrice
    }

    #[storage]
    struct Storage {
        finder: IFinderDispatcher,
        verified_prices: LegacyMap::<felt252, Price>,
        query_indices: LegacyMap::<felt252, QueryIndex>,
        requested_prices_len: u128,
        requested_prices: LegacyMap::<u128, QueryPoint>,
    }


    #[constructor]
    fn constructor(ref self: ContractState, finder: ContractAddress,) {
        self.finder.write(IFinderDispatcher { contract_address: finder });
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
                self
                    .query_indices
                    .write(
                        request_id,
                        QueryIndex {
                            is_valid: true, index: Option::Some(self.requested_prices_len.read())
                        }
                    );
                let requested_price_index = self.requested_prices_len.read();
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
            // TODO: Check if storing index as 0 is not problematic for the process (in such case, replace by Option::None)
            self
                .query_indices
                .write(request_id, QueryIndex { is_valid: false, index: Option::None });
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
        let input: Array<felt252> = array![identifier, time.high.into(), time.low.into()];
        let mut ancillary_data_felt: Array<felt252> = convert_byte_array_to_felt_array(
            ancillary_data
        );
        input.concat(@ancillary_data_felt);
        poseidon_hash_span(input.span())
    }
}
