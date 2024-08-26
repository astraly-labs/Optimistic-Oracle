#[starknet::contract]
pub mod mock_oracle {
    use pragma_lib::abi::{IPragmaABIDispatcher, IPragmaABIDispatcherTrait, IPragmaABI};
    use pragma_lib::types::{
        DataType, PragmaPricesResponse, AggregationMode, Currency, Pair, PossibleEntries,
        SimpleDataType, Checkpoint, SpotEntry, BaseEntry
    };
    use starknet::{ClassHash, ContractAddress};
    pub const DEFAULT_PRICE: u128 = 250000000000;
    #[storage]
    struct Storage {}
    #[abi(embed_v0)]
    impl IPragmaABIImpl of IPragmaABI<ContractState> {
        fn get_decimals(self: @ContractState, data_type: DataType) -> u32 {
            0
        }

        fn get_data_median(self: @ContractState, data_type: DataType) -> PragmaPricesResponse {
            PragmaPricesResponse {
                price: DEFAULT_PRICE,
                decimals: 8,
                last_updated_timestamp: starknet::get_block_timestamp(),
                num_sources_aggregated: 1,
                expiration_timestamp: Option::None
            }
        }

        fn get_data_median_for_sources(
            self: @ContractState, data_type: DataType, sources: Span<felt252>
        ) -> PragmaPricesResponse {
            PragmaPricesResponse {
                price: 0,
                decimals: 0,
                last_updated_timestamp: 0,
                num_sources_aggregated: 0,
                expiration_timestamp: Option::None
            }
        }

        fn get_data(
            self: @ContractState, data_type: DataType, aggregation_mode: AggregationMode
        ) -> PragmaPricesResponse {
            PragmaPricesResponse {
                price: 0,
                decimals: 0,
                last_updated_timestamp: 0,
                num_sources_aggregated: 0,
                expiration_timestamp: Option::None
            }
        }

        fn get_data_median_multi(
            self: @ContractState, data_types: Span<DataType>, sources: Span<felt252>
        ) -> Span<PragmaPricesResponse> {
            array![].span()
        }

        fn get_data_entry(
            self: @ContractState, data_type: DataType, source: felt252
        ) -> PossibleEntries {
            PossibleEntries::Spot(
                SpotEntry {
                    base: BaseEntry { timestamp: 0, source: source, publisher: 0 },
                    pair_id: 0,
                    price: 0,
                    volume: 0
                }
            )
        }


        fn get_data_for_sources(
            self: @ContractState,
            data_type: DataType,
            aggregation_mode: AggregationMode,
            sources: Span<felt252>
        ) -> PragmaPricesResponse {
            PragmaPricesResponse {
                price: 0,
                decimals: 0,
                last_updated_timestamp: 0,
                num_sources_aggregated: 0,
                expiration_timestamp: Option::None
            }
        }

        fn get_data_entries(self: @ContractState, data_type: DataType) -> Span<PossibleEntries> {
            array![].span()
        }

        fn get_data_entries_for_sources(
            self: @ContractState, data_type: DataType, sources: Span<felt252>
        ) -> (Span<PossibleEntries>, u64) {
            (array![].span(), 0)
        }

        fn get_last_checkpoint_before(
            self: @ContractState,
            data_type: DataType,
            timestamp: u64,
            aggregation_mode: AggregationMode,
        ) -> (Checkpoint, u64) {
            (
                Checkpoint {
                    timestamp: 0,
                    value: 0,
                    aggregation_mode: AggregationMode::Median,
                    num_sources_aggregated: 0
                },
                0
            )
        }

        fn get_data_with_USD_hop(
            self: @ContractState,
            base_currency_id: felt252,
            quote_currency_id: felt252,
            aggregation_mode: AggregationMode,
            typeof: SimpleDataType,
            expiration_timestamp: Option::<u64>
        ) -> PragmaPricesResponse {
            PragmaPricesResponse {
                price: 0,
                decimals: 0,
                last_updated_timestamp: 0,
                num_sources_aggregated: 0,
                expiration_timestamp: Option::None
            }
        }

        fn get_publisher_registry_address(self: @ContractState) -> ContractAddress {
            starknet::contract_address_const::<0>()
        }

        fn get_latest_checkpoint_index(
            self: @ContractState, data_type: DataType, aggregation_mode: AggregationMode
        ) -> (u64, bool) {
            (0, false)
        }

        fn get_latest_checkpoint(
            self: @ContractState, data_type: DataType, aggregation_mode: AggregationMode
        ) -> Checkpoint {
            Checkpoint {
                timestamp: 0,
                value: 0,
                aggregation_mode: AggregationMode::Median,
                num_sources_aggregated: 0
            }
        }

        fn get_checkpoint(
            self: @ContractState,
            data_type: DataType,
            checkpoint_index: u64,
            aggregation_mode: AggregationMode
        ) -> Checkpoint {
            Checkpoint {
                timestamp: 0,
                value: 0,
                aggregation_mode: AggregationMode::Median,
                num_sources_aggregated: 0
            }
        }

        fn get_sources_threshold(self: @ContractState,) -> u32 {
            0
        }

        fn get_admin_address(self: @ContractState,) -> ContractAddress {
            starknet::contract_address_const::<0>()
        }


        fn publish_data(ref self: ContractState, new_entry: PossibleEntries) {}

        fn publish_data_entries(ref self: ContractState, new_entries: Span<PossibleEntries>) {}

        fn set_admin_address(ref self: ContractState, new_admin_address: ContractAddress) {}

        fn update_publisher_registry_address(
            ref self: ContractState, new_publisher_registry_address: ContractAddress
        ) {}

        fn add_currency(ref self: ContractState, new_currency: Currency) {}

        fn update_currency(ref self: ContractState, currency_id: felt252, currency: Currency) {}


        fn add_pair(ref self: ContractState, new_pair: Pair) {}

        fn set_checkpoint(
            ref self: ContractState, data_type: DataType, aggregation_mode: AggregationMode
        ) {}

        fn set_checkpoints(
            ref self: ContractState, data_types: Span<DataType>, aggregation_mode: AggregationMode
        ) {}


        fn set_sources_threshold(ref self: ContractState, threshold: u32) {}

        fn upgrade(ref self: ContractState, impl_hash: ClassHash) {}

        fn get_implementation_hash(self: @ContractState) -> ClassHash {
            0.try_into().unwrap()
        }
    }
}
