#[derive(Debug)]
pub struct mock_oracle_ancillary<A: starknet::accounts::ConnectedAccount + Sync> {
    pub address: starknet::core::types::Felt,
    pub account: A,
    pub block_id: starknet::core::types::BlockId,
}
impl<A: starknet::accounts::ConnectedAccount + Sync> mock_oracle_ancillary<A> {
    pub fn new(address: starknet::core::types::Felt, account: A) -> Self {
        Self {
            address,
            account,
            block_id: starknet::core::types::BlockId::Tag(
                starknet::core::types::BlockTag::Pending,
            ),
        }
    }
    pub fn set_contract_address(&mut self, address: starknet::core::types::Felt) {
        self.address = address;
    }
    pub fn provider(&self) -> &A::Provider {
        self.account.provider()
    }
    pub fn set_block(&mut self, block_id: starknet::core::types::BlockId) {
        self.block_id = block_id;
    }
    pub fn with_block(self, block_id: starknet::core::types::BlockId) -> Self {
        Self { block_id, ..self }
    }
}
#[derive(Debug)]
pub struct mock_oracle_ancillaryReader<P: starknet::providers::Provider + Sync> {
    pub address: starknet::core::types::Felt,
    pub provider: P,
    pub block_id: starknet::core::types::BlockId,
}
impl<P: starknet::providers::Provider + Sync> mock_oracle_ancillaryReader<P> {
    pub fn new(address: starknet::core::types::Felt, provider: P) -> Self {
        Self {
            address,
            provider,
            block_id: starknet::core::types::BlockId::Tag(
                starknet::core::types::BlockTag::Pending,
            ),
        }
    }
    pub fn set_contract_address(&mut self, address: starknet::core::types::Felt) {
        self.address = address;
    }
    pub fn provider(&self) -> &P {
        &self.provider
    }
    pub fn set_block(&mut self, block_id: starknet::core::types::BlockId) {
        self.block_id = block_id;
    }
    pub fn with_block(self, block_id: starknet::core::types::BlockId) -> Self {
        Self { block_id, ..self }
    }
}
#[derive(Debug, PartialEq, PartialOrd, Clone, serde::Serialize, serde::Deserialize)]
pub struct PushedPrice {
    pub pusher: cainome::cairo_serde::ContractAddress,
    pub identifier: starknet::core::types::Felt,
    pub time: cainome::cairo_serde::U256,
    pub ancillary_data: cainome::cairo_serde::ByteArray,
    pub price: cainome::cairo_serde::U256,
    pub request_id: starknet::core::types::Felt,
}
impl cainome::cairo_serde::CairoSerde for PushedPrice {
    type RustType = Self;
    const SERIALIZED_SIZE: std::option::Option<usize> = None;
    #[inline]
    fn cairo_serialized_size(__rust: &Self::RustType) -> usize {
        let mut __size = 0;
        __size
            += cainome::cairo_serde::ContractAddress::cairo_serialized_size(
                &__rust.pusher,
            );
        __size += starknet::core::types::Felt::cairo_serialized_size(&__rust.identifier);
        __size += cainome::cairo_serde::U256::cairo_serialized_size(&__rust.time);
        __size
            += cainome::cairo_serde::ByteArray::cairo_serialized_size(
                &__rust.ancillary_data,
            );
        __size += cainome::cairo_serde::U256::cairo_serialized_size(&__rust.price);
        __size += starknet::core::types::Felt::cairo_serialized_size(&__rust.request_id);
        __size
    }
    fn cairo_serialize(__rust: &Self::RustType) -> Vec<starknet::core::types::Felt> {
        let mut __out: Vec<starknet::core::types::Felt> = vec![];
        __out
            .extend(
                cainome::cairo_serde::ContractAddress::cairo_serialize(&__rust.pusher),
            );
        __out.extend(starknet::core::types::Felt::cairo_serialize(&__rust.identifier));
        __out.extend(cainome::cairo_serde::U256::cairo_serialize(&__rust.time));
        __out
            .extend(
                cainome::cairo_serde::ByteArray::cairo_serialize(&__rust.ancillary_data),
            );
        __out.extend(cainome::cairo_serde::U256::cairo_serialize(&__rust.price));
        __out.extend(starknet::core::types::Felt::cairo_serialize(&__rust.request_id));
        __out
    }
    fn cairo_deserialize(
        __felts: &[starknet::core::types::Felt],
        __offset: usize,
    ) -> cainome::cairo_serde::Result<Self::RustType> {
        let mut __offset = __offset;
        let pusher = cainome::cairo_serde::ContractAddress::cairo_deserialize(
            __felts,
            __offset,
        )?;
        __offset
            += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&pusher);
        let identifier = starknet::core::types::Felt::cairo_deserialize(
            __felts,
            __offset,
        )?;
        __offset += starknet::core::types::Felt::cairo_serialized_size(&identifier);
        let time = cainome::cairo_serde::U256::cairo_deserialize(__felts, __offset)?;
        __offset += cainome::cairo_serde::U256::cairo_serialized_size(&time);
        let ancillary_data = cainome::cairo_serde::ByteArray::cairo_deserialize(
            __felts,
            __offset,
        )?;
        __offset
            += cainome::cairo_serde::ByteArray::cairo_serialized_size(&ancillary_data);
        let price = cainome::cairo_serde::U256::cairo_deserialize(__felts, __offset)?;
        __offset += cainome::cairo_serde::U256::cairo_serialized_size(&price);
        let request_id = starknet::core::types::Felt::cairo_deserialize(
            __felts,
            __offset,
        )?;
        __offset += starknet::core::types::Felt::cairo_serialized_size(&request_id);
        Ok(PushedPrice {
            pusher,
            identifier,
            time,
            ancillary_data,
            price,
            request_id,
        })
    }
}
#[derive(Debug, PartialEq, PartialOrd, Clone, serde::Serialize, serde::Deserialize)]
pub struct PriceRequestAdded {
    pub requester: cainome::cairo_serde::ContractAddress,
    pub identifier: starknet::core::types::Felt,
    pub time: cainome::cairo_serde::U256,
    pub ancillary_data: cainome::cairo_serde::ByteArray,
    pub request_id: starknet::core::types::Felt,
}
impl cainome::cairo_serde::CairoSerde for PriceRequestAdded {
    type RustType = Self;
    const SERIALIZED_SIZE: std::option::Option<usize> = None;
    #[inline]
    fn cairo_serialized_size(__rust: &Self::RustType) -> usize {
        let mut __size = 0;
        __size
            += cainome::cairo_serde::ContractAddress::cairo_serialized_size(
                &__rust.requester,
            );
        __size += starknet::core::types::Felt::cairo_serialized_size(&__rust.identifier);
        __size += cainome::cairo_serde::U256::cairo_serialized_size(&__rust.time);
        __size
            += cainome::cairo_serde::ByteArray::cairo_serialized_size(
                &__rust.ancillary_data,
            );
        __size += starknet::core::types::Felt::cairo_serialized_size(&__rust.request_id);
        __size
    }
    fn cairo_serialize(__rust: &Self::RustType) -> Vec<starknet::core::types::Felt> {
        let mut __out: Vec<starknet::core::types::Felt> = vec![];
        __out
            .extend(
                cainome::cairo_serde::ContractAddress::cairo_serialize(&__rust.requester),
            );
        __out.extend(starknet::core::types::Felt::cairo_serialize(&__rust.identifier));
        __out.extend(cainome::cairo_serde::U256::cairo_serialize(&__rust.time));
        __out
            .extend(
                cainome::cairo_serde::ByteArray::cairo_serialize(&__rust.ancillary_data),
            );
        __out.extend(starknet::core::types::Felt::cairo_serialize(&__rust.request_id));
        __out
    }
    fn cairo_deserialize(
        __felts: &[starknet::core::types::Felt],
        __offset: usize,
    ) -> cainome::cairo_serde::Result<Self::RustType> {
        let mut __offset = __offset;
        let requester = cainome::cairo_serde::ContractAddress::cairo_deserialize(
            __felts,
            __offset,
        )?;
        __offset
            += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&requester);
        let identifier = starknet::core::types::Felt::cairo_deserialize(
            __felts,
            __offset,
        )?;
        __offset += starknet::core::types::Felt::cairo_serialized_size(&identifier);
        let time = cainome::cairo_serde::U256::cairo_deserialize(__felts, __offset)?;
        __offset += cainome::cairo_serde::U256::cairo_serialized_size(&time);
        let ancillary_data = cainome::cairo_serde::ByteArray::cairo_deserialize(
            __felts,
            __offset,
        )?;
        __offset
            += cainome::cairo_serde::ByteArray::cairo_serialized_size(&ancillary_data);
        let request_id = starknet::core::types::Felt::cairo_deserialize(
            __felts,
            __offset,
        )?;
        __offset += starknet::core::types::Felt::cairo_serialized_size(&request_id);
        Ok(PriceRequestAdded {
            requester,
            identifier,
            time,
            ancillary_data,
            request_id,
        })
    }
}
#[derive(Debug, PartialEq, PartialOrd, Clone, serde::Serialize, serde::Deserialize)]
pub struct IIdentifierWhitelistDispatcher {
    pub contract_address: cainome::cairo_serde::ContractAddress,
}
impl cainome::cairo_serde::CairoSerde for IIdentifierWhitelistDispatcher {
    type RustType = Self;
    const SERIALIZED_SIZE: std::option::Option<usize> = None;
    #[inline]
    fn cairo_serialized_size(__rust: &Self::RustType) -> usize {
        let mut __size = 0;
        __size
            += cainome::cairo_serde::ContractAddress::cairo_serialized_size(
                &__rust.contract_address,
            );
        __size
    }
    fn cairo_serialize(__rust: &Self::RustType) -> Vec<starknet::core::types::Felt> {
        let mut __out: Vec<starknet::core::types::Felt> = vec![];
        __out
            .extend(
                cainome::cairo_serde::ContractAddress::cairo_serialize(
                    &__rust.contract_address,
                ),
            );
        __out
    }
    fn cairo_deserialize(
        __felts: &[starknet::core::types::Felt],
        __offset: usize,
    ) -> cainome::cairo_serde::Result<Self::RustType> {
        let mut __offset = __offset;
        let contract_address = cainome::cairo_serde::ContractAddress::cairo_deserialize(
            __felts,
            __offset,
        )?;
        __offset
            += cainome::cairo_serde::ContractAddress::cairo_serialized_size(
                &contract_address,
            );
        Ok(IIdentifierWhitelistDispatcher {
            contract_address,
        })
    }
}
#[derive(Debug, PartialEq, PartialOrd, Clone, serde::Serialize, serde::Deserialize)]
pub struct QueryPoint {
    pub identifier: starknet::core::types::Felt,
    pub time: cainome::cairo_serde::U256,
    pub ancillary_data: cainome::cairo_serde::ByteArray,
}
impl cainome::cairo_serde::CairoSerde for QueryPoint {
    type RustType = Self;
    const SERIALIZED_SIZE: std::option::Option<usize> = None;
    #[inline]
    fn cairo_serialized_size(__rust: &Self::RustType) -> usize {
        let mut __size = 0;
        __size += starknet::core::types::Felt::cairo_serialized_size(&__rust.identifier);
        __size += cainome::cairo_serde::U256::cairo_serialized_size(&__rust.time);
        __size
            += cainome::cairo_serde::ByteArray::cairo_serialized_size(
                &__rust.ancillary_data,
            );
        __size
    }
    fn cairo_serialize(__rust: &Self::RustType) -> Vec<starknet::core::types::Felt> {
        let mut __out: Vec<starknet::core::types::Felt> = vec![];
        __out.extend(starknet::core::types::Felt::cairo_serialize(&__rust.identifier));
        __out.extend(cainome::cairo_serde::U256::cairo_serialize(&__rust.time));
        __out
            .extend(
                cainome::cairo_serde::ByteArray::cairo_serialize(&__rust.ancillary_data),
            );
        __out
    }
    fn cairo_deserialize(
        __felts: &[starknet::core::types::Felt],
        __offset: usize,
    ) -> cainome::cairo_serde::Result<Self::RustType> {
        let mut __offset = __offset;
        let identifier = starknet::core::types::Felt::cairo_deserialize(
            __felts,
            __offset,
        )?;
        __offset += starknet::core::types::Felt::cairo_serialized_size(&identifier);
        let time = cainome::cairo_serde::U256::cairo_deserialize(__felts, __offset)?;
        __offset += cainome::cairo_serde::U256::cairo_serialized_size(&time);
        let ancillary_data = cainome::cairo_serde::ByteArray::cairo_deserialize(
            __felts,
            __offset,
        )?;
        __offset
            += cainome::cairo_serde::ByteArray::cairo_serialized_size(&ancillary_data);
        Ok(QueryPoint {
            identifier,
            time,
            ancillary_data,
        })
    }
}
#[derive(Debug, PartialEq, PartialOrd, Clone, serde::Serialize, serde::Deserialize)]
pub enum Event {
    PriceRequestAdded(PriceRequestAdded),
    PushedPrice(PushedPrice),
}
impl cainome::cairo_serde::CairoSerde for Event {
    type RustType = Self;
    const SERIALIZED_SIZE: std::option::Option<usize> = std::option::Option::None;
    #[inline]
    fn cairo_serialized_size(__rust: &Self::RustType) -> usize {
        match __rust {
            Event::PriceRequestAdded(val) => {
                PriceRequestAdded::cairo_serialized_size(val) + 1
            }
            Event::PushedPrice(val) => PushedPrice::cairo_serialized_size(val) + 1,
            _ => 0,
        }
    }
    fn cairo_serialize(__rust: &Self::RustType) -> Vec<starknet::core::types::Felt> {
        match __rust {
            Event::PriceRequestAdded(val) => {
                let mut temp = vec![];
                temp.extend(usize::cairo_serialize(&0usize));
                temp.extend(PriceRequestAdded::cairo_serialize(val));
                temp
            }
            Event::PushedPrice(val) => {
                let mut temp = vec![];
                temp.extend(usize::cairo_serialize(&1usize));
                temp.extend(PushedPrice::cairo_serialize(val));
                temp
            }
            _ => vec![],
        }
    }
    fn cairo_deserialize(
        __felts: &[starknet::core::types::Felt],
        __offset: usize,
    ) -> cainome::cairo_serde::Result<Self::RustType> {
        let __f = __felts[__offset];
        let __index = u128::from_be_bytes(__f.to_bytes_be()[16..].try_into().unwrap());
        match __index as usize {
            0usize => {
                Ok(
                    Event::PriceRequestAdded(
                        PriceRequestAdded::cairo_deserialize(__felts, __offset + 1)?,
                    ),
                )
            }
            1usize => {
                Ok(
                    Event::PushedPrice(
                        PushedPrice::cairo_deserialize(__felts, __offset + 1)?,
                    ),
                )
            }
            _ => {
                return Err(
                    cainome::cairo_serde::Error::Deserialize(
                        format!("Index not handle for enum {}", "Event"),
                    ),
                );
            }
        }
    }
}
impl TryFrom<starknet::core::types::EmittedEvent> for Event {
    type Error = String;
    fn try_from(
        event: starknet::core::types::EmittedEvent,
    ) -> Result<Self, Self::Error> {
        use cainome::cairo_serde::CairoSerde;
        if event.keys.is_empty() {
            return Err("Event has no key".to_string());
        }
        let selector = event.keys[0];
        if selector
            == starknet::core::utils::get_selector_from_name("PriceRequestAdded")
                .unwrap_or_else(|_| {
                    panic!("Invalid selector for {}", "PriceRequestAdded")
                })
        {
            let mut key_offset = 0 + 1;
            let mut data_offset = 0;
            let requester = match cainome::cairo_serde::ContractAddress::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(
                        format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "requester",
                            "PriceRequestAdded",
                            e,
                        ),
                    );
                }
            };
            data_offset
                += cainome::cairo_serde::ContractAddress::cairo_serialized_size(
                    &requester,
                );
            let identifier = match starknet::core::types::Felt::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(
                        format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "identifier",
                            "PriceRequestAdded",
                            e,
                        ),
                    );
                }
            };
            data_offset
                += starknet::core::types::Felt::cairo_serialized_size(&identifier);
            let time = match cainome::cairo_serde::U256::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(
                        format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "time",
                            "PriceRequestAdded",
                            e,
                        ),
                    );
                }
            };
            data_offset += cainome::cairo_serde::U256::cairo_serialized_size(&time);
            let ancillary_data = match cainome::cairo_serde::ByteArray::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(
                        format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "ancillary_data",
                            "PriceRequestAdded",
                            e,
                        ),
                    );
                }
            };
            data_offset
                += cainome::cairo_serde::ByteArray::cairo_serialized_size(
                    &ancillary_data,
                );
            let request_id = match starknet::core::types::Felt::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(
                        format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "request_id",
                            "PriceRequestAdded",
                            e,
                        ),
                    );
                }
            };
            data_offset
                += starknet::core::types::Felt::cairo_serialized_size(&request_id);
            return Ok(
                Event::PriceRequestAdded(PriceRequestAdded {
                    requester,
                    identifier,
                    time,
                    ancillary_data,
                    request_id,
                }),
            );
        }
        let selector = event.keys[0];
        if selector
            == starknet::core::utils::get_selector_from_name("PushedPrice")
                .unwrap_or_else(|_| panic!("Invalid selector for {}", "PushedPrice"))
        {
            let mut key_offset = 0 + 1;
            let mut data_offset = 0;
            let pusher = match cainome::cairo_serde::ContractAddress::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(
                        format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "pusher",
                            "PushedPrice",
                            e,
                        ),
                    );
                }
            };
            data_offset
                += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&pusher);
            let identifier = match starknet::core::types::Felt::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(
                        format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "identifier",
                            "PushedPrice",
                            e,
                        ),
                    );
                }
            };
            data_offset
                += starknet::core::types::Felt::cairo_serialized_size(&identifier);
            let time = match cainome::cairo_serde::U256::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(
                        format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "time",
                            "PushedPrice",
                            e,
                        ),
                    );
                }
            };
            data_offset += cainome::cairo_serde::U256::cairo_serialized_size(&time);
            let ancillary_data = match cainome::cairo_serde::ByteArray::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(
                        format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "ancillary_data",
                            "PushedPrice",
                            e,
                        ),
                    );
                }
            };
            data_offset
                += cainome::cairo_serde::ByteArray::cairo_serialized_size(
                    &ancillary_data,
                );
            let price = match cainome::cairo_serde::U256::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(
                        format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "price",
                            "PushedPrice",
                            e,
                        ),
                    );
                }
            };
            data_offset += cainome::cairo_serde::U256::cairo_serialized_size(&price);
            let request_id = match starknet::core::types::Felt::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(
                        format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "request_id",
                            "PushedPrice",
                            e,
                        ),
                    );
                }
            };
            data_offset
                += starknet::core::types::Felt::cairo_serialized_size(&request_id);
            return Ok(
                Event::PushedPrice(PushedPrice {
                    pusher,
                    identifier,
                    time,
                    ancillary_data,
                    price,
                    request_id,
                }),
            );
        }
        Err(format!("Could not match any event from keys {:?}", event.keys))
    }
}
impl<A: starknet::accounts::ConnectedAccount + Sync> mock_oracle_ancillary<A> {
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn has_price(
        &self,
        identifier: &starknet::core::types::Felt,
        time: &cainome::cairo_serde::U256,
        ancillary_data: &cainome::cairo_serde::ByteArray,
    ) -> cainome::cairo_serde::call::FCall<A::Provider, bool> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(starknet::core::types::Felt::cairo_serialize(identifier));
        __calldata.extend(cainome::cairo_serde::U256::cairo_serialize(time));
        __calldata
            .extend(cainome::cairo_serde::ByteArray::cairo_serialize(ancillary_data));
        let __call = starknet::core::types::FunctionCall {
            contract_address: self.address,
            entry_point_selector: starknet::macros::selector!("has_price"),
            calldata: __calldata,
        };
        cainome::cairo_serde::call::FCall::new(__call, self.provider())
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn get_price(
        &self,
        identifier: &starknet::core::types::Felt,
        time: &cainome::cairo_serde::U256,
        ancillary_data: &cainome::cairo_serde::ByteArray,
    ) -> cainome::cairo_serde::call::FCall<A::Provider, cainome::cairo_serde::U256> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(starknet::core::types::Felt::cairo_serialize(identifier));
        __calldata.extend(cainome::cairo_serde::U256::cairo_serialize(time));
        __calldata
            .extend(cainome::cairo_serde::ByteArray::cairo_serialize(ancillary_data));
        let __call = starknet::core::types::FunctionCall {
            contract_address: self.address,
            entry_point_selector: starknet::macros::selector!("get_price"),
            calldata: __calldata,
        };
        cainome::cairo_serde::call::FCall::new(__call, self.provider())
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn get_identifier_whitelist(
        &self,
    ) -> cainome::cairo_serde::call::FCall<A::Provider, IIdentifierWhitelistDispatcher> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        let __call = starknet::core::types::FunctionCall {
            contract_address: self.address,
            entry_point_selector: starknet::macros::selector!(
                "get_identifier_whitelist"
            ),
            calldata: __calldata,
        };
        cainome::cairo_serde::call::FCall::new(__call, self.provider())
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn get_pending_queries(
        &self,
    ) -> cainome::cairo_serde::call::FCall<A::Provider, Vec<QueryPoint>> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        let __call = starknet::core::types::FunctionCall {
            contract_address: self.address,
            entry_point_selector: starknet::macros::selector!("get_pending_queries"),
            calldata: __calldata,
        };
        cainome::cairo_serde::call::FCall::new(__call, self.provider())
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn get_request_parameters(
        &self,
        request_id: &starknet::core::types::Felt,
    ) -> cainome::cairo_serde::call::FCall<A::Provider, QueryPoint> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(starknet::core::types::Felt::cairo_serialize(request_id));
        let __call = starknet::core::types::FunctionCall {
            contract_address: self.address,
            entry_point_selector: starknet::macros::selector!("get_request_parameters"),
            calldata: __calldata,
        };
        cainome::cairo_serde::call::FCall::new(__call, self.provider())
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn request_price_getcall(
        &self,
        identifier: &starknet::core::types::Felt,
        time: &cainome::cairo_serde::U256,
        ancillary_data: &cainome::cairo_serde::ByteArray,
    ) -> starknet::accounts::Call {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(starknet::core::types::Felt::cairo_serialize(identifier));
        __calldata.extend(cainome::cairo_serde::U256::cairo_serialize(time));
        __calldata
            .extend(cainome::cairo_serde::ByteArray::cairo_serialize(ancillary_data));
        starknet::accounts::Call {
            to: self.address,
            selector: starknet::macros::selector!("request_price"),
            calldata: __calldata,
        }
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn request_price(
        &self,
        identifier: &starknet::core::types::Felt,
        time: &cainome::cairo_serde::U256,
        ancillary_data: &cainome::cairo_serde::ByteArray,
    ) -> starknet::accounts::ExecutionV1<A> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(starknet::core::types::Felt::cairo_serialize(identifier));
        __calldata.extend(cainome::cairo_serde::U256::cairo_serialize(time));
        __calldata
            .extend(cainome::cairo_serde::ByteArray::cairo_serialize(ancillary_data));
        let __call = starknet::accounts::Call {
            to: self.address,
            selector: starknet::macros::selector!("request_price"),
            calldata: __calldata,
        };
        self.account.execute_v1(vec![__call])
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn push_price_getcall(
        &self,
        identifier: &starknet::core::types::Felt,
        time: &cainome::cairo_serde::U256,
        ancillary_data: &cainome::cairo_serde::ByteArray,
        price: &cainome::cairo_serde::U256,
    ) -> starknet::accounts::Call {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(starknet::core::types::Felt::cairo_serialize(identifier));
        __calldata.extend(cainome::cairo_serde::U256::cairo_serialize(time));
        __calldata
            .extend(cainome::cairo_serde::ByteArray::cairo_serialize(ancillary_data));
        __calldata.extend(cainome::cairo_serde::U256::cairo_serialize(price));
        starknet::accounts::Call {
            to: self.address,
            selector: starknet::macros::selector!("push_price"),
            calldata: __calldata,
        }
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn push_price(
        &self,
        identifier: &starknet::core::types::Felt,
        time: &cainome::cairo_serde::U256,
        ancillary_data: &cainome::cairo_serde::ByteArray,
        price: &cainome::cairo_serde::U256,
    ) -> starknet::accounts::ExecutionV1<A> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(starknet::core::types::Felt::cairo_serialize(identifier));
        __calldata.extend(cainome::cairo_serde::U256::cairo_serialize(time));
        __calldata
            .extend(cainome::cairo_serde::ByteArray::cairo_serialize(ancillary_data));
        __calldata.extend(cainome::cairo_serde::U256::cairo_serialize(price));
        let __call = starknet::accounts::Call {
            to: self.address,
            selector: starknet::macros::selector!("push_price"),
            calldata: __calldata,
        };
        self.account.execute_v1(vec![__call])
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn push_price_by_request_id_getcall(
        &self,
        request_id: &starknet::core::types::Felt,
        price: &cainome::cairo_serde::U256,
    ) -> starknet::accounts::Call {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(starknet::core::types::Felt::cairo_serialize(request_id));
        __calldata.extend(cainome::cairo_serde::U256::cairo_serialize(price));
        starknet::accounts::Call {
            to: self.address,
            selector: starknet::macros::selector!("push_price_by_request_id"),
            calldata: __calldata,
        }
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn push_price_by_request_id(
        &self,
        request_id: &starknet::core::types::Felt,
        price: &cainome::cairo_serde::U256,
    ) -> starknet::accounts::ExecutionV1<A> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(starknet::core::types::Felt::cairo_serialize(request_id));
        __calldata.extend(cainome::cairo_serde::U256::cairo_serialize(price));
        let __call = starknet::accounts::Call {
            to: self.address,
            selector: starknet::macros::selector!("push_price_by_request_id"),
            calldata: __calldata,
        };
        self.account.execute_v1(vec![__call])
    }
}
impl<P: starknet::providers::Provider + Sync> mock_oracle_ancillaryReader<P> {
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn has_price(
        &self,
        identifier: &starknet::core::types::Felt,
        time: &cainome::cairo_serde::U256,
        ancillary_data: &cainome::cairo_serde::ByteArray,
    ) -> cainome::cairo_serde::call::FCall<P, bool> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(starknet::core::types::Felt::cairo_serialize(identifier));
        __calldata.extend(cainome::cairo_serde::U256::cairo_serialize(time));
        __calldata
            .extend(cainome::cairo_serde::ByteArray::cairo_serialize(ancillary_data));
        let __call = starknet::core::types::FunctionCall {
            contract_address: self.address,
            entry_point_selector: starknet::macros::selector!("has_price"),
            calldata: __calldata,
        };
        cainome::cairo_serde::call::FCall::new(__call, self.provider())
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn get_price(
        &self,
        identifier: &starknet::core::types::Felt,
        time: &cainome::cairo_serde::U256,
        ancillary_data: &cainome::cairo_serde::ByteArray,
    ) -> cainome::cairo_serde::call::FCall<P, cainome::cairo_serde::U256> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(starknet::core::types::Felt::cairo_serialize(identifier));
        __calldata.extend(cainome::cairo_serde::U256::cairo_serialize(time));
        __calldata
            .extend(cainome::cairo_serde::ByteArray::cairo_serialize(ancillary_data));
        let __call = starknet::core::types::FunctionCall {
            contract_address: self.address,
            entry_point_selector: starknet::macros::selector!("get_price"),
            calldata: __calldata,
        };
        cainome::cairo_serde::call::FCall::new(__call, self.provider())
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn get_identifier_whitelist(
        &self,
    ) -> cainome::cairo_serde::call::FCall<P, IIdentifierWhitelistDispatcher> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        let __call = starknet::core::types::FunctionCall {
            contract_address: self.address,
            entry_point_selector: starknet::macros::selector!(
                "get_identifier_whitelist"
            ),
            calldata: __calldata,
        };
        cainome::cairo_serde::call::FCall::new(__call, self.provider())
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn get_pending_queries(
        &self,
    ) -> cainome::cairo_serde::call::FCall<P, Vec<QueryPoint>> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        let __call = starknet::core::types::FunctionCall {
            contract_address: self.address,
            entry_point_selector: starknet::macros::selector!("get_pending_queries"),
            calldata: __calldata,
        };
        cainome::cairo_serde::call::FCall::new(__call, self.provider())
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn get_request_parameters(
        &self,
        request_id: &starknet::core::types::Felt,
    ) -> cainome::cairo_serde::call::FCall<P, QueryPoint> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(starknet::core::types::Felt::cairo_serialize(request_id));
        let __call = starknet::core::types::FunctionCall {
            contract_address: self.address,
            entry_point_selector: starknet::macros::selector!("get_request_parameters"),
            calldata: __calldata,
        };
        cainome::cairo_serde::call::FCall::new(__call, self.provider())
    }
}
