#[derive(Debug)]
pub struct store<A: starknet::accounts::ConnectedAccount + Sync> {
    pub address: starknet::core::types::FieldElement,
    pub account: A,
    pub block_id: starknet::core::types::BlockId,
}
impl<A: starknet::accounts::ConnectedAccount + Sync> store<A> {
    pub fn new(address: starknet::core::types::FieldElement, account: A) -> Self {
        Self {
            address,
            account,
            block_id: starknet::core::types::BlockId::Tag(
                starknet::core::types::BlockTag::Pending,
            ),
        }
    }
    pub fn set_contract_address(mut self, address: starknet::core::types::FieldElement) {
        self.address = address;
    }
    pub fn provider(&self) -> &A::Provider {
        self.account.provider()
    }
    pub fn set_block(mut self, block_id: starknet::core::types::BlockId) {
        self.block_id = block_id;
    }
}
#[derive(Debug)]
pub struct storeReader<P: starknet::providers::Provider + Sync> {
    pub address: starknet::core::types::FieldElement,
    pub provider: P,
    pub block_id: starknet::core::types::BlockId,
}
impl<P: starknet::providers::Provider + Sync> storeReader<P> {
    pub fn new(address: starknet::core::types::FieldElement, provider: P) -> Self {
        Self {
            address,
            provider,
            block_id: starknet::core::types::BlockId::Tag(
                starknet::core::types::BlockTag::Pending,
            ),
        }
    }
    pub fn set_contract_address(mut self, address: starknet::core::types::FieldElement) {
        self.address = address;
    }
    pub fn provider(&self) -> &P {
        &self.provider
    }
    pub fn set_block(mut self, block_id: starknet::core::types::BlockId) {
        self.block_id = block_id;
    }
}
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct OwnershipTransferred {
    pub previous_owner: cainome::cairo_serde::ContractAddress,
    pub new_owner: cainome::cairo_serde::ContractAddress,
}
impl cainome::cairo_serde::CairoSerde for OwnershipTransferred {
    type RustType = Self;
    const SERIALIZED_SIZE: std::option::Option<usize> = None;
    #[inline]
    fn cairo_serialized_size(__rust: &Self::RustType) -> usize {
        let mut __size = 0;
        __size
            += cainome::cairo_serde::ContractAddress::cairo_serialized_size(
                &__rust.previous_owner,
            );
        __size
            += cainome::cairo_serde::ContractAddress::cairo_serialized_size(
                &__rust.new_owner,
            );
        __size
    }
    fn cairo_serialize(
        __rust: &Self::RustType,
    ) -> Vec<starknet::core::types::FieldElement> {
        let mut __out: Vec<starknet::core::types::FieldElement> = vec![];
        __out
            .extend(
                cainome::cairo_serde::ContractAddress::cairo_serialize(
                    &__rust.previous_owner,
                ),
            );
        __out
            .extend(
                cainome::cairo_serde::ContractAddress::cairo_serialize(&__rust.new_owner),
            );
        __out
    }
    fn cairo_deserialize(
        __felts: &[starknet::core::types::FieldElement],
        __offset: usize,
    ) -> cainome::cairo_serde::Result<Self::RustType> {
        let mut __offset = __offset;
        let previous_owner = cainome::cairo_serde::ContractAddress::cairo_deserialize(
            __felts,
            __offset,
        )?;
        __offset
            += cainome::cairo_serde::ContractAddress::cairo_serialized_size(
                &previous_owner,
            );
        let new_owner = cainome::cairo_serde::ContractAddress::cairo_deserialize(
            __felts,
            __offset,
        )?;
        __offset
            += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&new_owner);
        Ok(OwnershipTransferred {
            previous_owner,
            new_owner,
        })
    }
}
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct NewFinalFee {
    pub new_final_fee: cainome::cairo_serde::U256,
}
impl cainome::cairo_serde::CairoSerde for NewFinalFee {
    type RustType = Self;
    const SERIALIZED_SIZE: std::option::Option<usize> = None;
    #[inline]
    fn cairo_serialized_size(__rust: &Self::RustType) -> usize {
        let mut __size = 0;
        __size
            += cainome::cairo_serde::U256::cairo_serialized_size(&__rust.new_final_fee);
        __size
    }
    fn cairo_serialize(
        __rust: &Self::RustType,
    ) -> Vec<starknet::core::types::FieldElement> {
        let mut __out: Vec<starknet::core::types::FieldElement> = vec![];
        __out.extend(cainome::cairo_serde::U256::cairo_serialize(&__rust.new_final_fee));
        __out
    }
    fn cairo_deserialize(
        __felts: &[starknet::core::types::FieldElement],
        __offset: usize,
    ) -> cainome::cairo_serde::Result<Self::RustType> {
        let mut __offset = __offset;
        let new_final_fee = cainome::cairo_serde::U256::cairo_deserialize(
            __felts,
            __offset,
        )?;
        __offset += cainome::cairo_serde::U256::cairo_serialized_size(&new_final_fee);
        Ok(NewFinalFee { new_final_fee })
    }
}
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct OwnershipTransferStarted {
    pub previous_owner: cainome::cairo_serde::ContractAddress,
    pub new_owner: cainome::cairo_serde::ContractAddress,
}
impl cainome::cairo_serde::CairoSerde for OwnershipTransferStarted {
    type RustType = Self;
    const SERIALIZED_SIZE: std::option::Option<usize> = None;
    #[inline]
    fn cairo_serialized_size(__rust: &Self::RustType) -> usize {
        let mut __size = 0;
        __size
            += cainome::cairo_serde::ContractAddress::cairo_serialized_size(
                &__rust.previous_owner,
            );
        __size
            += cainome::cairo_serde::ContractAddress::cairo_serialized_size(
                &__rust.new_owner,
            );
        __size
    }
    fn cairo_serialize(
        __rust: &Self::RustType,
    ) -> Vec<starknet::core::types::FieldElement> {
        let mut __out: Vec<starknet::core::types::FieldElement> = vec![];
        __out
            .extend(
                cainome::cairo_serde::ContractAddress::cairo_serialize(
                    &__rust.previous_owner,
                ),
            );
        __out
            .extend(
                cainome::cairo_serde::ContractAddress::cairo_serialize(&__rust.new_owner),
            );
        __out
    }
    fn cairo_deserialize(
        __felts: &[starknet::core::types::FieldElement],
        __offset: usize,
    ) -> cainome::cairo_serde::Result<Self::RustType> {
        let mut __offset = __offset;
        let previous_owner = cainome::cairo_serde::ContractAddress::cairo_deserialize(
            __felts,
            __offset,
        )?;
        __offset
            += cainome::cairo_serde::ContractAddress::cairo_serialized_size(
                &previous_owner,
            );
        let new_owner = cainome::cairo_serde::ContractAddress::cairo_deserialize(
            __felts,
            __offset,
        )?;
        __offset
            += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&new_owner);
        Ok(OwnershipTransferStarted {
            previous_owner,
            new_owner,
        })
    }
}
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Event {
    OwnershipTransferred(OwnershipTransferred),
    OwnershipTransferStarted(OwnershipTransferStarted),
}
impl cainome::cairo_serde::CairoSerde for Event {
    type RustType = Self;
    const SERIALIZED_SIZE: std::option::Option<usize> = std::option::Option::None;
    #[inline]
    fn cairo_serialized_size(__rust: &Self::RustType) -> usize {
        match __rust {
            Event::OwnershipTransferred(val) => {
                OwnershipTransferred::cairo_serialized_size(val) + 1
            }
            Event::OwnershipTransferStarted(val) => {
                OwnershipTransferStarted::cairo_serialized_size(val) + 1
            }
            _ => 0,
        }
    }
    fn cairo_serialize(
        __rust: &Self::RustType,
    ) -> Vec<starknet::core::types::FieldElement> {
        match __rust {
            Event::OwnershipTransferred(val) => {
                let mut temp = vec![];
                temp.extend(usize::cairo_serialize(&0usize));
                temp.extend(OwnershipTransferred::cairo_serialize(val));
                temp
            }
            Event::OwnershipTransferStarted(val) => {
                let mut temp = vec![];
                temp.extend(usize::cairo_serialize(&1usize));
                temp.extend(OwnershipTransferStarted::cairo_serialize(val));
                temp
            }
            _ => vec![],
        }
    }
    fn cairo_deserialize(
        __felts: &[starknet::core::types::FieldElement],
        __offset: usize,
    ) -> cainome::cairo_serde::Result<Self::RustType> {
        let __index: u128 = __felts[__offset].try_into().unwrap();
        match __index as usize {
            0usize => {
                Ok(
                    Event::OwnershipTransferred(
                        OwnershipTransferred::cairo_deserialize(__felts, __offset + 1)?,
                    ),
                )
            }
            1usize => {
                Ok(
                    Event::OwnershipTransferStarted(
                        OwnershipTransferStarted::cairo_deserialize(
                            __felts,
                            __offset + 1,
                        )?,
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
            == starknet::core::utils::get_selector_from_name("OwnershipTransferred")
                .unwrap_or_else(|_| {
                    panic!("Invalid selector for {}", "OwnershipTransferred")
                })
        {
            let mut key_offset = 0 + 1;
            let mut data_offset = 0;
            let previous_owner = match cainome::cairo_serde::ContractAddress::cairo_deserialize(
                &event.keys,
                key_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(
                        format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "previous_owner",
                            "OwnershipTransferred",
                            e,
                        ),
                    );
                }
            };
            key_offset
                += cainome::cairo_serde::ContractAddress::cairo_serialized_size(
                    &previous_owner,
                );
            let new_owner = match cainome::cairo_serde::ContractAddress::cairo_deserialize(
                &event.keys,
                key_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(
                        format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "new_owner",
                            "OwnershipTransferred",
                            e,
                        ),
                    );
                }
            };
            key_offset
                += cainome::cairo_serde::ContractAddress::cairo_serialized_size(
                    &new_owner,
                );
            return Ok(
                Event::OwnershipTransferred(OwnershipTransferred {
                    previous_owner,
                    new_owner,
                }),
            );
        }
        let selector = event.keys[0];
        if selector
            == starknet::core::utils::get_selector_from_name("OwnershipTransferStarted")
                .unwrap_or_else(|_| {
                    panic!("Invalid selector for {}", "OwnershipTransferStarted")
                })
        {
            let mut key_offset = 0 + 1;
            let mut data_offset = 0;
            let previous_owner = match cainome::cairo_serde::ContractAddress::cairo_deserialize(
                &event.keys,
                key_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(
                        format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "previous_owner",
                            "OwnershipTransferStarted",
                            e,
                        ),
                    );
                }
            };
            key_offset
                += cainome::cairo_serde::ContractAddress::cairo_serialized_size(
                    &previous_owner,
                );
            let new_owner = match cainome::cairo_serde::ContractAddress::cairo_deserialize(
                &event.keys,
                key_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(
                        format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "new_owner",
                            "OwnershipTransferStarted",
                            e,
                        ),
                    );
                }
            };
            key_offset
                += cainome::cairo_serde::ContractAddress::cairo_serialized_size(
                    &new_owner,
                );
            return Ok(
                Event::OwnershipTransferStarted(OwnershipTransferStarted {
                    previous_owner,
                    new_owner,
                }),
            );
        }
        Err(format!("Could not match any event from keys {:?}", event.keys))
    }
}
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Event {
    NewFinalFee(NewFinalFee),
    OwnableEvent(Event),
}
impl cainome::cairo_serde::CairoSerde for Event {
    type RustType = Self;
    const SERIALIZED_SIZE: std::option::Option<usize> = std::option::Option::None;
    #[inline]
    fn cairo_serialized_size(__rust: &Self::RustType) -> usize {
        match __rust {
            Event::NewFinalFee(val) => NewFinalFee::cairo_serialized_size(val) + 1,
            Event::OwnableEvent(val) => Event::cairo_serialized_size(val) + 1,
            _ => 0,
        }
    }
    fn cairo_serialize(
        __rust: &Self::RustType,
    ) -> Vec<starknet::core::types::FieldElement> {
        match __rust {
            Event::NewFinalFee(val) => {
                let mut temp = vec![];
                temp.extend(usize::cairo_serialize(&0usize));
                temp.extend(NewFinalFee::cairo_serialize(val));
                temp
            }
            Event::OwnableEvent(val) => {
                let mut temp = vec![];
                temp.extend(usize::cairo_serialize(&1usize));
                temp.extend(Event::cairo_serialize(val));
                temp
            }
            _ => vec![],
        }
    }
    fn cairo_deserialize(
        __felts: &[starknet::core::types::FieldElement],
        __offset: usize,
    ) -> cainome::cairo_serde::Result<Self::RustType> {
        let __index: u128 = __felts[__offset].try_into().unwrap();
        match __index as usize {
            0usize => {
                Ok(
                    Event::NewFinalFee(
                        NewFinalFee::cairo_deserialize(__felts, __offset + 1)?,
                    ),
                )
            }
            1usize => {
                Ok(Event::OwnableEvent(Event::cairo_deserialize(__felts, __offset + 1)?))
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
            == starknet::core::utils::get_selector_from_name("NewFinalFee")
                .unwrap_or_else(|_| panic!("Invalid selector for {}", "NewFinalFee"))
        {
            let mut key_offset = 0 + 1;
            let mut data_offset = 0;
            let new_final_fee = match cainome::cairo_serde::U256::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(
                        format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "new_final_fee",
                            "NewFinalFee",
                            e,
                        ),
                    );
                }
            };
            data_offset
                += cainome::cairo_serde::U256::cairo_serialized_size(&new_final_fee);
            return Ok(Event::NewFinalFee(NewFinalFee { new_final_fee }));
        }
        let selector = event.keys[0];
        if selector
            == starknet::core::utils::get_selector_from_name("OwnershipTransferred")
                .unwrap_or_else(|_| {
                    panic!("Invalid selector for {}", "OwnershipTransferred")
                })
        {
            let mut key_offset = 0 + 1;
            let mut data_offset = 0;
            let previous_owner = match cainome::cairo_serde::ContractAddress::cairo_deserialize(
                &event.keys,
                key_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(
                        format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "previous_owner",
                            "OwnershipTransferred",
                            e,
                        ),
                    );
                }
            };
            key_offset
                += cainome::cairo_serde::ContractAddress::cairo_serialized_size(
                    &previous_owner,
                );
            let new_owner = match cainome::cairo_serde::ContractAddress::cairo_deserialize(
                &event.keys,
                key_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(
                        format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "new_owner",
                            "OwnershipTransferred",
                            e,
                        ),
                    );
                }
            };
            key_offset
                += cainome::cairo_serde::ContractAddress::cairo_serialized_size(
                    &new_owner,
                );
            return Ok(
                Event::OwnableEvent(
                    Event::OwnershipTransferred(OwnershipTransferred {
                        previous_owner,
                        new_owner,
                    }),
                ),
            );
        }
        let selector = event.keys[0];
        if selector
            == starknet::core::utils::get_selector_from_name("OwnershipTransferStarted")
                .unwrap_or_else(|_| {
                    panic!("Invalid selector for {}", "OwnershipTransferStarted")
                })
        {
            let mut key_offset = 0 + 1;
            let mut data_offset = 0;
            let previous_owner = match cainome::cairo_serde::ContractAddress::cairo_deserialize(
                &event.keys,
                key_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(
                        format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "previous_owner",
                            "OwnershipTransferStarted",
                            e,
                        ),
                    );
                }
            };
            key_offset
                += cainome::cairo_serde::ContractAddress::cairo_serialized_size(
                    &previous_owner,
                );
            let new_owner = match cainome::cairo_serde::ContractAddress::cairo_deserialize(
                &event.keys,
                key_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(
                        format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "new_owner",
                            "OwnershipTransferStarted",
                            e,
                        ),
                    );
                }
            };
            key_offset
                += cainome::cairo_serde::ContractAddress::cairo_serialized_size(
                    &new_owner,
                );
            return Ok(
                Event::OwnableEvent(
                    Event::OwnershipTransferStarted(OwnershipTransferStarted {
                        previous_owner,
                        new_owner,
                    }),
                ),
            );
        }
        Err(format!("Could not match any event from keys {:?}", event.keys))
    }
}
impl<A: starknet::accounts::ConnectedAccount + Sync> store<A> {
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn pay_oracle_fees(
        &self,
        erc20_address: &cainome::cairo_serde::ContractAddress,
        amount: &cainome::cairo_serde::U256,
    ) -> cainome::cairo_serde::call::FCall<A::Provider, ()> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata
            .extend(
                cainome::cairo_serde::ContractAddress::cairo_serialize(erc20_address),
            );
        __calldata.extend(cainome::cairo_serde::U256::cairo_serialize(amount));
        let __call = starknet::core::types::FunctionCall {
            contract_address: self.address,
            entry_point_selector: starknet::macros::selector!("pay_oracle_fees"),
            calldata: __calldata,
        };
        cainome::cairo_serde::call::FCall::new(__call, self.provider())
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn compute_final_fee(
        &self,
        currency: &cainome::cairo_serde::ContractAddress,
    ) -> cainome::cairo_serde::call::FCall<A::Provider, cainome::cairo_serde::U256> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata
            .extend(cainome::cairo_serde::ContractAddress::cairo_serialize(currency));
        let __call = starknet::core::types::FunctionCall {
            contract_address: self.address,
            entry_point_selector: starknet::macros::selector!("compute_final_fee"),
            calldata: __calldata,
        };
        cainome::cairo_serde::call::FCall::new(__call, self.provider())
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn owner(
        &self,
    ) -> cainome::cairo_serde::call::FCall<
        A::Provider,
        cainome::cairo_serde::ContractAddress,
    > {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        let __call = starknet::core::types::FunctionCall {
            contract_address: self.address,
            entry_point_selector: starknet::macros::selector!("owner"),
            calldata: __calldata,
        };
        cainome::cairo_serde::call::FCall::new(__call, self.provider())
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn set_final_fee_getcall(
        &self,
        currency: &cainome::cairo_serde::ContractAddress,
        new_final_fee: &cainome::cairo_serde::U256,
    ) -> starknet::accounts::Call {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata
            .extend(cainome::cairo_serde::ContractAddress::cairo_serialize(currency));
        __calldata.extend(cainome::cairo_serde::U256::cairo_serialize(new_final_fee));
        starknet::accounts::Call {
            to: self.address,
            selector: starknet::macros::selector!("set_final_fee"),
            calldata: __calldata,
        }
    }
    #[allow(clippy::ptr_arg)]
    pub fn set_final_fee(
        &self,
        currency: &cainome::cairo_serde::ContractAddress,
        new_final_fee: &cainome::cairo_serde::U256,
    ) -> starknet::accounts::Execution<A> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata
            .extend(cainome::cairo_serde::ContractAddress::cairo_serialize(currency));
        __calldata.extend(cainome::cairo_serde::U256::cairo_serialize(new_final_fee));
        let __call = starknet::accounts::Call {
            to: self.address,
            selector: starknet::macros::selector!("set_final_fee"),
            calldata: __calldata,
        };
        self.account.execute(vec![__call])
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn transfer_ownership_getcall(
        &self,
        new_owner: &cainome::cairo_serde::ContractAddress,
    ) -> starknet::accounts::Call {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata
            .extend(cainome::cairo_serde::ContractAddress::cairo_serialize(new_owner));
        starknet::accounts::Call {
            to: self.address,
            selector: starknet::macros::selector!("transfer_ownership"),
            calldata: __calldata,
        }
    }
    #[allow(clippy::ptr_arg)]
    pub fn transfer_ownership(
        &self,
        new_owner: &cainome::cairo_serde::ContractAddress,
    ) -> starknet::accounts::Execution<A> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata
            .extend(cainome::cairo_serde::ContractAddress::cairo_serialize(new_owner));
        let __call = starknet::accounts::Call {
            to: self.address,
            selector: starknet::macros::selector!("transfer_ownership"),
            calldata: __calldata,
        };
        self.account.execute(vec![__call])
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn renounce_ownership_getcall(&self) -> starknet::accounts::Call {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        starknet::accounts::Call {
            to: self.address,
            selector: starknet::macros::selector!("renounce_ownership"),
            calldata: __calldata,
        }
    }
    #[allow(clippy::ptr_arg)]
    pub fn renounce_ownership(&self) -> starknet::accounts::Execution<A> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        let __call = starknet::accounts::Call {
            to: self.address,
            selector: starknet::macros::selector!("renounce_ownership"),
            calldata: __calldata,
        };
        self.account.execute(vec![__call])
    }
}
impl<P: starknet::providers::Provider + Sync> storeReader<P> {
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn pay_oracle_fees(
        &self,
        erc20_address: &cainome::cairo_serde::ContractAddress,
        amount: &cainome::cairo_serde::U256,
    ) -> cainome::cairo_serde::call::FCall<P, ()> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata
            .extend(
                cainome::cairo_serde::ContractAddress::cairo_serialize(erc20_address),
            );
        __calldata.extend(cainome::cairo_serde::U256::cairo_serialize(amount));
        let __call = starknet::core::types::FunctionCall {
            contract_address: self.address,
            entry_point_selector: starknet::macros::selector!("pay_oracle_fees"),
            calldata: __calldata,
        };
        cainome::cairo_serde::call::FCall::new(__call, self.provider())
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn compute_final_fee(
        &self,
        currency: &cainome::cairo_serde::ContractAddress,
    ) -> cainome::cairo_serde::call::FCall<P, cainome::cairo_serde::U256> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata
            .extend(cainome::cairo_serde::ContractAddress::cairo_serialize(currency));
        let __call = starknet::core::types::FunctionCall {
            contract_address: self.address,
            entry_point_selector: starknet::macros::selector!("compute_final_fee"),
            calldata: __calldata,
        };
        cainome::cairo_serde::call::FCall::new(__call, self.provider())
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn owner(
        &self,
    ) -> cainome::cairo_serde::call::FCall<P, cainome::cairo_serde::ContractAddress> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        let __call = starknet::core::types::FunctionCall {
            contract_address: self.address,
            entry_point_selector: starknet::macros::selector!("owner"),
            calldata: __calldata,
        };
        cainome::cairo_serde::call::FCall::new(__call, self.provider())
    }
}
