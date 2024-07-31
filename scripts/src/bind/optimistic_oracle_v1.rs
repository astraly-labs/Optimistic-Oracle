#[derive(Debug)]
pub struct optimistic_oracle_v1<A: starknet::accounts::ConnectedAccount + Sync> {
    pub address: starknet::core::types::FieldElement,
    pub account: A,
    pub block_id: starknet::core::types::BlockId,
}
impl<A: starknet::accounts::ConnectedAccount + Sync> optimistic_oracle_v1<A> {
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
pub struct optimistic_oracle_v1Reader<P: starknet::providers::Provider + Sync> {
    pub address: starknet::core::types::FieldElement,
    pub provider: P,
    pub block_id: starknet::core::types::BlockId,
}
impl<P: starknet::providers::Provider + Sync> optimistic_oracle_v1Reader<P> {
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
pub struct AdminPropertiesSet {
    pub default_currency: cainome::cairo_serde::ContractAddress,
    pub default_liveness: u64,
    pub burned_bond_percentage: cainome::cairo_serde::U256,
}
impl cainome::cairo_serde::CairoSerde for AdminPropertiesSet {
    type RustType = Self;
    const SERIALIZED_SIZE: std::option::Option<usize> = None;
    #[inline]
    fn cairo_serialized_size(__rust: &Self::RustType) -> usize {
        let mut __size = 0;
        __size
            += cainome::cairo_serde::ContractAddress::cairo_serialized_size(
                &__rust.default_currency,
            );
        __size += u64::cairo_serialized_size(&__rust.default_liveness);
        __size
            += cainome::cairo_serde::U256::cairo_serialized_size(
                &__rust.burned_bond_percentage,
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
                    &__rust.default_currency,
                ),
            );
        __out.extend(u64::cairo_serialize(&__rust.default_liveness));
        __out
            .extend(
                cainome::cairo_serde::U256::cairo_serialize(
                    &__rust.burned_bond_percentage,
                ),
            );
        __out
    }
    fn cairo_deserialize(
        __felts: &[starknet::core::types::FieldElement],
        __offset: usize,
    ) -> cainome::cairo_serde::Result<Self::RustType> {
        let mut __offset = __offset;
        let default_currency = cainome::cairo_serde::ContractAddress::cairo_deserialize(
            __felts,
            __offset,
        )?;
        __offset
            += cainome::cairo_serde::ContractAddress::cairo_serialized_size(
                &default_currency,
            );
        let default_liveness = u64::cairo_deserialize(__felts, __offset)?;
        __offset += u64::cairo_serialized_size(&default_liveness);
        let burned_bond_percentage = cainome::cairo_serde::U256::cairo_deserialize(
            __felts,
            __offset,
        )?;
        __offset
            += cainome::cairo_serde::U256::cairo_serialized_size(
                &burned_bond_percentage,
            );
        Ok(AdminPropertiesSet {
            default_currency,
            default_liveness,
            burned_bond_percentage,
        })
    }
}
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct ERC20ABIDispatcher {
    pub contract_address: cainome::cairo_serde::ContractAddress,
}
impl cainome::cairo_serde::CairoSerde for ERC20ABIDispatcher {
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
    fn cairo_serialize(
        __rust: &Self::RustType,
    ) -> Vec<starknet::core::types::FieldElement> {
        let mut __out: Vec<starknet::core::types::FieldElement> = vec![];
        __out
            .extend(
                cainome::cairo_serde::ContractAddress::cairo_serialize(
                    &__rust.contract_address,
                ),
            );
        __out
    }
    fn cairo_deserialize(
        __felts: &[starknet::core::types::FieldElement],
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
        Ok(ERC20ABIDispatcher {
            contract_address,
        })
    }
}
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct EscalationManagerSettings {
    pub arbitrate_via_escalation_manager: bool,
    pub discard_oracle: bool,
    pub validate_disputers: bool,
    pub asserting_caller: cainome::cairo_serde::ContractAddress,
    pub escalation_manager: cainome::cairo_serde::ContractAddress,
}
impl cainome::cairo_serde::CairoSerde for EscalationManagerSettings {
    type RustType = Self;
    const SERIALIZED_SIZE: std::option::Option<usize> = None;
    #[inline]
    fn cairo_serialized_size(__rust: &Self::RustType) -> usize {
        let mut __size = 0;
        __size += bool::cairo_serialized_size(&__rust.arbitrate_via_escalation_manager);
        __size += bool::cairo_serialized_size(&__rust.discard_oracle);
        __size += bool::cairo_serialized_size(&__rust.validate_disputers);
        __size
            += cainome::cairo_serde::ContractAddress::cairo_serialized_size(
                &__rust.asserting_caller,
            );
        __size
            += cainome::cairo_serde::ContractAddress::cairo_serialized_size(
                &__rust.escalation_manager,
            );
        __size
    }
    fn cairo_serialize(
        __rust: &Self::RustType,
    ) -> Vec<starknet::core::types::FieldElement> {
        let mut __out: Vec<starknet::core::types::FieldElement> = vec![];
        __out.extend(bool::cairo_serialize(&__rust.arbitrate_via_escalation_manager));
        __out.extend(bool::cairo_serialize(&__rust.discard_oracle));
        __out.extend(bool::cairo_serialize(&__rust.validate_disputers));
        __out
            .extend(
                cainome::cairo_serde::ContractAddress::cairo_serialize(
                    &__rust.asserting_caller,
                ),
            );
        __out
            .extend(
                cainome::cairo_serde::ContractAddress::cairo_serialize(
                    &__rust.escalation_manager,
                ),
            );
        __out
    }
    fn cairo_deserialize(
        __felts: &[starknet::core::types::FieldElement],
        __offset: usize,
    ) -> cainome::cairo_serde::Result<Self::RustType> {
        let mut __offset = __offset;
        let arbitrate_via_escalation_manager = bool::cairo_deserialize(
            __felts,
            __offset,
        )?;
        __offset += bool::cairo_serialized_size(&arbitrate_via_escalation_manager);
        let discard_oracle = bool::cairo_deserialize(__felts, __offset)?;
        __offset += bool::cairo_serialized_size(&discard_oracle);
        let validate_disputers = bool::cairo_deserialize(__felts, __offset)?;
        __offset += bool::cairo_serialized_size(&validate_disputers);
        let asserting_caller = cainome::cairo_serde::ContractAddress::cairo_deserialize(
            __felts,
            __offset,
        )?;
        __offset
            += cainome::cairo_serde::ContractAddress::cairo_serialized_size(
                &asserting_caller,
            );
        let escalation_manager = cainome::cairo_serde::ContractAddress::cairo_deserialize(
            __felts,
            __offset,
        )?;
        __offset
            += cainome::cairo_serde::ContractAddress::cairo_serialized_size(
                &escalation_manager,
            );
        Ok(EscalationManagerSettings {
            arbitrate_via_escalation_manager,
            discard_oracle,
            validate_disputers,
            asserting_caller,
            escalation_manager,
        })
    }
}
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct AssertionMade {
    pub assertion_id: starknet::core::types::FieldElement,
    pub domain_id: cainome::cairo_serde::U256,
    pub claim: cainome::cairo_serde::ByteArray,
    pub asserter: cainome::cairo_serde::ContractAddress,
    pub callback_recipient: cainome::cairo_serde::ContractAddress,
    pub escalation_manager: cainome::cairo_serde::ContractAddress,
    pub caller: cainome::cairo_serde::ContractAddress,
    pub expiration_timestamp: u64,
    pub currency: ERC20ABIDispatcher,
    pub bond: cainome::cairo_serde::U256,
    pub identifier: starknet::core::types::FieldElement,
}
impl cainome::cairo_serde::CairoSerde for AssertionMade {
    type RustType = Self;
    const SERIALIZED_SIZE: std::option::Option<usize> = None;
    #[inline]
    fn cairo_serialized_size(__rust: &Self::RustType) -> usize {
        let mut __size = 0;
        __size
            += starknet::core::types::FieldElement::cairo_serialized_size(
                &__rust.assertion_id,
            );
        __size += cainome::cairo_serde::U256::cairo_serialized_size(&__rust.domain_id);
        __size += cainome::cairo_serde::ByteArray::cairo_serialized_size(&__rust.claim);
        __size
            += cainome::cairo_serde::ContractAddress::cairo_serialized_size(
                &__rust.asserter,
            );
        __size
            += cainome::cairo_serde::ContractAddress::cairo_serialized_size(
                &__rust.callback_recipient,
            );
        __size
            += cainome::cairo_serde::ContractAddress::cairo_serialized_size(
                &__rust.escalation_manager,
            );
        __size
            += cainome::cairo_serde::ContractAddress::cairo_serialized_size(
                &__rust.caller,
            );
        __size += u64::cairo_serialized_size(&__rust.expiration_timestamp);
        __size += ERC20ABIDispatcher::cairo_serialized_size(&__rust.currency);
        __size += cainome::cairo_serde::U256::cairo_serialized_size(&__rust.bond);
        __size
            += starknet::core::types::FieldElement::cairo_serialized_size(
                &__rust.identifier,
            );
        __size
    }
    fn cairo_serialize(
        __rust: &Self::RustType,
    ) -> Vec<starknet::core::types::FieldElement> {
        let mut __out: Vec<starknet::core::types::FieldElement> = vec![];
        __out
            .extend(
                starknet::core::types::FieldElement::cairo_serialize(
                    &__rust.assertion_id,
                ),
            );
        __out.extend(cainome::cairo_serde::U256::cairo_serialize(&__rust.domain_id));
        __out.extend(cainome::cairo_serde::ByteArray::cairo_serialize(&__rust.claim));
        __out
            .extend(
                cainome::cairo_serde::ContractAddress::cairo_serialize(&__rust.asserter),
            );
        __out
            .extend(
                cainome::cairo_serde::ContractAddress::cairo_serialize(
                    &__rust.callback_recipient,
                ),
            );
        __out
            .extend(
                cainome::cairo_serde::ContractAddress::cairo_serialize(
                    &__rust.escalation_manager,
                ),
            );
        __out
            .extend(
                cainome::cairo_serde::ContractAddress::cairo_serialize(&__rust.caller),
            );
        __out.extend(u64::cairo_serialize(&__rust.expiration_timestamp));
        __out.extend(ERC20ABIDispatcher::cairo_serialize(&__rust.currency));
        __out.extend(cainome::cairo_serde::U256::cairo_serialize(&__rust.bond));
        __out
            .extend(
                starknet::core::types::FieldElement::cairo_serialize(&__rust.identifier),
            );
        __out
    }
    fn cairo_deserialize(
        __felts: &[starknet::core::types::FieldElement],
        __offset: usize,
    ) -> cainome::cairo_serde::Result<Self::RustType> {
        let mut __offset = __offset;
        let assertion_id = starknet::core::types::FieldElement::cairo_deserialize(
            __felts,
            __offset,
        )?;
        __offset
            += starknet::core::types::FieldElement::cairo_serialized_size(&assertion_id);
        let domain_id = cainome::cairo_serde::U256::cairo_deserialize(
            __felts,
            __offset,
        )?;
        __offset += cainome::cairo_serde::U256::cairo_serialized_size(&domain_id);
        let claim = cainome::cairo_serde::ByteArray::cairo_deserialize(
            __felts,
            __offset,
        )?;
        __offset += cainome::cairo_serde::ByteArray::cairo_serialized_size(&claim);
        let asserter = cainome::cairo_serde::ContractAddress::cairo_deserialize(
            __felts,
            __offset,
        )?;
        __offset
            += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&asserter);
        let callback_recipient = cainome::cairo_serde::ContractAddress::cairo_deserialize(
            __felts,
            __offset,
        )?;
        __offset
            += cainome::cairo_serde::ContractAddress::cairo_serialized_size(
                &callback_recipient,
            );
        let escalation_manager = cainome::cairo_serde::ContractAddress::cairo_deserialize(
            __felts,
            __offset,
        )?;
        __offset
            += cainome::cairo_serde::ContractAddress::cairo_serialized_size(
                &escalation_manager,
            );
        let caller = cainome::cairo_serde::ContractAddress::cairo_deserialize(
            __felts,
            __offset,
        )?;
        __offset
            += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&caller);
        let expiration_timestamp = u64::cairo_deserialize(__felts, __offset)?;
        __offset += u64::cairo_serialized_size(&expiration_timestamp);
        let currency = ERC20ABIDispatcher::cairo_deserialize(__felts, __offset)?;
        __offset += ERC20ABIDispatcher::cairo_serialized_size(&currency);
        let bond = cainome::cairo_serde::U256::cairo_deserialize(__felts, __offset)?;
        __offset += cainome::cairo_serde::U256::cairo_serialized_size(&bond);
        let identifier = starknet::core::types::FieldElement::cairo_deserialize(
            __felts,
            __offset,
        )?;
        __offset
            += starknet::core::types::FieldElement::cairo_serialized_size(&identifier);
        Ok(AssertionMade {
            assertion_id,
            domain_id,
            claim,
            asserter,
            callback_recipient,
            escalation_manager,
            caller,
            expiration_timestamp,
            currency,
            bond,
            identifier,
        })
    }
}
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct AssertionSettled {
    pub assertion_id: starknet::core::types::FieldElement,
    pub bond_recipient: cainome::cairo_serde::ContractAddress,
    pub disputed: bool,
    pub settlement_resolution: bool,
    pub settle_caller: cainome::cairo_serde::ContractAddress,
}
impl cainome::cairo_serde::CairoSerde for AssertionSettled {
    type RustType = Self;
    const SERIALIZED_SIZE: std::option::Option<usize> = None;
    #[inline]
    fn cairo_serialized_size(__rust: &Self::RustType) -> usize {
        let mut __size = 0;
        __size
            += starknet::core::types::FieldElement::cairo_serialized_size(
                &__rust.assertion_id,
            );
        __size
            += cainome::cairo_serde::ContractAddress::cairo_serialized_size(
                &__rust.bond_recipient,
            );
        __size += bool::cairo_serialized_size(&__rust.disputed);
        __size += bool::cairo_serialized_size(&__rust.settlement_resolution);
        __size
            += cainome::cairo_serde::ContractAddress::cairo_serialized_size(
                &__rust.settle_caller,
            );
        __size
    }
    fn cairo_serialize(
        __rust: &Self::RustType,
    ) -> Vec<starknet::core::types::FieldElement> {
        let mut __out: Vec<starknet::core::types::FieldElement> = vec![];
        __out
            .extend(
                starknet::core::types::FieldElement::cairo_serialize(
                    &__rust.assertion_id,
                ),
            );
        __out
            .extend(
                cainome::cairo_serde::ContractAddress::cairo_serialize(
                    &__rust.bond_recipient,
                ),
            );
        __out.extend(bool::cairo_serialize(&__rust.disputed));
        __out.extend(bool::cairo_serialize(&__rust.settlement_resolution));
        __out
            .extend(
                cainome::cairo_serde::ContractAddress::cairo_serialize(
                    &__rust.settle_caller,
                ),
            );
        __out
    }
    fn cairo_deserialize(
        __felts: &[starknet::core::types::FieldElement],
        __offset: usize,
    ) -> cainome::cairo_serde::Result<Self::RustType> {
        let mut __offset = __offset;
        let assertion_id = starknet::core::types::FieldElement::cairo_deserialize(
            __felts,
            __offset,
        )?;
        __offset
            += starknet::core::types::FieldElement::cairo_serialized_size(&assertion_id);
        let bond_recipient = cainome::cairo_serde::ContractAddress::cairo_deserialize(
            __felts,
            __offset,
        )?;
        __offset
            += cainome::cairo_serde::ContractAddress::cairo_serialized_size(
                &bond_recipient,
            );
        let disputed = bool::cairo_deserialize(__felts, __offset)?;
        __offset += bool::cairo_serialized_size(&disputed);
        let settlement_resolution = bool::cairo_deserialize(__felts, __offset)?;
        __offset += bool::cairo_serialized_size(&settlement_resolution);
        let settle_caller = cainome::cairo_serde::ContractAddress::cairo_deserialize(
            __felts,
            __offset,
        )?;
        __offset
            += cainome::cairo_serde::ContractAddress::cairo_serialized_size(
                &settle_caller,
            );
        Ok(AssertionSettled {
            assertion_id,
            bond_recipient,
            disputed,
            settlement_resolution,
            settle_caller,
        })
    }
}
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Assertion {
    pub escalation_manager_settings: EscalationManagerSettings,
    pub asserter: cainome::cairo_serde::ContractAddress,
    pub assertion_time: u64,
    pub settled: bool,
    pub currency: ERC20ABIDispatcher,
    pub expiration_time: u64,
    pub settlement_resolution: bool,
    pub domain_id: cainome::cairo_serde::U256,
    pub identifier: starknet::core::types::FieldElement,
    pub bond: cainome::cairo_serde::U256,
    pub callback_recipient: cainome::cairo_serde::ContractAddress,
    pub disputer: cainome::cairo_serde::ContractAddress,
}
impl cainome::cairo_serde::CairoSerde for Assertion {
    type RustType = Self;
    const SERIALIZED_SIZE: std::option::Option<usize> = None;
    #[inline]
    fn cairo_serialized_size(__rust: &Self::RustType) -> usize {
        let mut __size = 0;
        __size
            += EscalationManagerSettings::cairo_serialized_size(
                &__rust.escalation_manager_settings,
            );
        __size
            += cainome::cairo_serde::ContractAddress::cairo_serialized_size(
                &__rust.asserter,
            );
        __size += u64::cairo_serialized_size(&__rust.assertion_time);
        __size += bool::cairo_serialized_size(&__rust.settled);
        __size += ERC20ABIDispatcher::cairo_serialized_size(&__rust.currency);
        __size += u64::cairo_serialized_size(&__rust.expiration_time);
        __size += bool::cairo_serialized_size(&__rust.settlement_resolution);
        __size += cainome::cairo_serde::U256::cairo_serialized_size(&__rust.domain_id);
        __size
            += starknet::core::types::FieldElement::cairo_serialized_size(
                &__rust.identifier,
            );
        __size += cainome::cairo_serde::U256::cairo_serialized_size(&__rust.bond);
        __size
            += cainome::cairo_serde::ContractAddress::cairo_serialized_size(
                &__rust.callback_recipient,
            );
        __size
            += cainome::cairo_serde::ContractAddress::cairo_serialized_size(
                &__rust.disputer,
            );
        __size
    }
    fn cairo_serialize(
        __rust: &Self::RustType,
    ) -> Vec<starknet::core::types::FieldElement> {
        let mut __out: Vec<starknet::core::types::FieldElement> = vec![];
        __out
            .extend(
                EscalationManagerSettings::cairo_serialize(
                    &__rust.escalation_manager_settings,
                ),
            );
        __out
            .extend(
                cainome::cairo_serde::ContractAddress::cairo_serialize(&__rust.asserter),
            );
        __out.extend(u64::cairo_serialize(&__rust.assertion_time));
        __out.extend(bool::cairo_serialize(&__rust.settled));
        __out.extend(ERC20ABIDispatcher::cairo_serialize(&__rust.currency));
        __out.extend(u64::cairo_serialize(&__rust.expiration_time));
        __out.extend(bool::cairo_serialize(&__rust.settlement_resolution));
        __out.extend(cainome::cairo_serde::U256::cairo_serialize(&__rust.domain_id));
        __out
            .extend(
                starknet::core::types::FieldElement::cairo_serialize(&__rust.identifier),
            );
        __out.extend(cainome::cairo_serde::U256::cairo_serialize(&__rust.bond));
        __out
            .extend(
                cainome::cairo_serde::ContractAddress::cairo_serialize(
                    &__rust.callback_recipient,
                ),
            );
        __out
            .extend(
                cainome::cairo_serde::ContractAddress::cairo_serialize(&__rust.disputer),
            );
        __out
    }
    fn cairo_deserialize(
        __felts: &[starknet::core::types::FieldElement],
        __offset: usize,
    ) -> cainome::cairo_serde::Result<Self::RustType> {
        let mut __offset = __offset;
        let escalation_manager_settings = EscalationManagerSettings::cairo_deserialize(
            __felts,
            __offset,
        )?;
        __offset
            += EscalationManagerSettings::cairo_serialized_size(
                &escalation_manager_settings,
            );
        let asserter = cainome::cairo_serde::ContractAddress::cairo_deserialize(
            __felts,
            __offset,
        )?;
        __offset
            += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&asserter);
        let assertion_time = u64::cairo_deserialize(__felts, __offset)?;
        __offset += u64::cairo_serialized_size(&assertion_time);
        let settled = bool::cairo_deserialize(__felts, __offset)?;
        __offset += bool::cairo_serialized_size(&settled);
        let currency = ERC20ABIDispatcher::cairo_deserialize(__felts, __offset)?;
        __offset += ERC20ABIDispatcher::cairo_serialized_size(&currency);
        let expiration_time = u64::cairo_deserialize(__felts, __offset)?;
        __offset += u64::cairo_serialized_size(&expiration_time);
        let settlement_resolution = bool::cairo_deserialize(__felts, __offset)?;
        __offset += bool::cairo_serialized_size(&settlement_resolution);
        let domain_id = cainome::cairo_serde::U256::cairo_deserialize(
            __felts,
            __offset,
        )?;
        __offset += cainome::cairo_serde::U256::cairo_serialized_size(&domain_id);
        let identifier = starknet::core::types::FieldElement::cairo_deserialize(
            __felts,
            __offset,
        )?;
        __offset
            += starknet::core::types::FieldElement::cairo_serialized_size(&identifier);
        let bond = cainome::cairo_serde::U256::cairo_deserialize(__felts, __offset)?;
        __offset += cainome::cairo_serde::U256::cairo_serialized_size(&bond);
        let callback_recipient = cainome::cairo_serde::ContractAddress::cairo_deserialize(
            __felts,
            __offset,
        )?;
        __offset
            += cainome::cairo_serde::ContractAddress::cairo_serialized_size(
                &callback_recipient,
            );
        let disputer = cainome::cairo_serde::ContractAddress::cairo_deserialize(
            __felts,
            __offset,
        )?;
        __offset
            += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&disputer);
        Ok(Assertion {
            escalation_manager_settings,
            asserter,
            assertion_time,
            settled,
            currency,
            expiration_time,
            settlement_resolution,
            domain_id,
            identifier,
            bond,
            callback_recipient,
            disputer,
        })
    }
}
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct AssertionDisputed {
    pub assertion_id: starknet::core::types::FieldElement,
    pub caller: cainome::cairo_serde::ContractAddress,
    pub disputer: cainome::cairo_serde::ContractAddress,
}
impl cainome::cairo_serde::CairoSerde for AssertionDisputed {
    type RustType = Self;
    const SERIALIZED_SIZE: std::option::Option<usize> = None;
    #[inline]
    fn cairo_serialized_size(__rust: &Self::RustType) -> usize {
        let mut __size = 0;
        __size
            += starknet::core::types::FieldElement::cairo_serialized_size(
                &__rust.assertion_id,
            );
        __size
            += cainome::cairo_serde::ContractAddress::cairo_serialized_size(
                &__rust.caller,
            );
        __size
            += cainome::cairo_serde::ContractAddress::cairo_serialized_size(
                &__rust.disputer,
            );
        __size
    }
    fn cairo_serialize(
        __rust: &Self::RustType,
    ) -> Vec<starknet::core::types::FieldElement> {
        let mut __out: Vec<starknet::core::types::FieldElement> = vec![];
        __out
            .extend(
                starknet::core::types::FieldElement::cairo_serialize(
                    &__rust.assertion_id,
                ),
            );
        __out
            .extend(
                cainome::cairo_serde::ContractAddress::cairo_serialize(&__rust.caller),
            );
        __out
            .extend(
                cainome::cairo_serde::ContractAddress::cairo_serialize(&__rust.disputer),
            );
        __out
    }
    fn cairo_deserialize(
        __felts: &[starknet::core::types::FieldElement],
        __offset: usize,
    ) -> cainome::cairo_serde::Result<Self::RustType> {
        let mut __offset = __offset;
        let assertion_id = starknet::core::types::FieldElement::cairo_deserialize(
            __felts,
            __offset,
        )?;
        __offset
            += starknet::core::types::FieldElement::cairo_serialized_size(&assertion_id);
        let caller = cainome::cairo_serde::ContractAddress::cairo_deserialize(
            __felts,
            __offset,
        )?;
        __offset
            += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&caller);
        let disputer = cainome::cairo_serde::ContractAddress::cairo_deserialize(
            __felts,
            __offset,
        )?;
        __offset
            += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&disputer);
        Ok(AssertionDisputed {
            assertion_id,
            caller,
            disputer,
        })
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
pub enum OwnableCptEvent {
    OwnershipTransferred(OwnershipTransferred),
    OwnershipTransferStarted(OwnershipTransferStarted),
}
impl cainome::cairo_serde::CairoSerde for OwnableCptEvent {
    type RustType = Self;
    const SERIALIZED_SIZE: std::option::Option<usize> = std::option::Option::None;
    #[inline]
    fn cairo_serialized_size(__rust: &Self::RustType) -> usize {
        match __rust {
            OwnableCptEvent::OwnershipTransferred(val) => {
                OwnershipTransferred::cairo_serialized_size(val) + 1
            }
            OwnableCptEvent::OwnershipTransferStarted(val) => {
                OwnershipTransferStarted::cairo_serialized_size(val) + 1
            }
            _ => 0,
        }
    }
    fn cairo_serialize(
        __rust: &Self::RustType,
    ) -> Vec<starknet::core::types::FieldElement> {
        match __rust {
            OwnableCptEvent::OwnershipTransferred(val) => {
                let mut temp = vec![];
                temp.extend(usize::cairo_serialize(&0usize));
                temp.extend(OwnershipTransferred::cairo_serialize(val));
                temp
            }
            OwnableCptEvent::OwnershipTransferStarted(val) => {
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
                    OwnableCptEvent::OwnershipTransferred(
                        OwnershipTransferred::cairo_deserialize(__felts, __offset + 1)?,
                    ),
                )
            }
            1usize => {
                Ok(
                    OwnableCptEvent::OwnershipTransferStarted(
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
                        format!("Index not handle for enum {}", "OwnableCptEvent"),
                    ),
                );
            }
        }
    }
}
impl TryFrom<starknet::core::types::EmittedEvent> for OwnableCptEvent {
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
                OwnableCptEvent::OwnershipTransferred(OwnershipTransferred {
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
                OwnableCptEvent::OwnershipTransferStarted(OwnershipTransferStarted {
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
    AdminPropertiesSet(AdminPropertiesSet),
    AssertionSettled(AssertionSettled),
    AssertionMade(AssertionMade),
    AssertionDisputed(AssertionDisputed),
    OwnableEvent(OwnableCptEvent),
    ReentrancyGuardEvent(Event),
}
impl cainome::cairo_serde::CairoSerde for Event {
    type RustType = Self;
    const SERIALIZED_SIZE: std::option::Option<usize> = std::option::Option::None;
    #[inline]
    fn cairo_serialized_size(__rust: &Self::RustType) -> usize {
        match __rust {
            Event::AdminPropertiesSet(val) => {
                AdminPropertiesSet::cairo_serialized_size(val) + 1
            }
            Event::AssertionSettled(val) => {
                AssertionSettled::cairo_serialized_size(val) + 1
            }
            Event::AssertionMade(val) => AssertionMade::cairo_serialized_size(val) + 1,
            Event::AssertionDisputed(val) => {
                AssertionDisputed::cairo_serialized_size(val) + 1
            }
            Event::OwnableEvent(val) => OwnableCptEvent::cairo_serialized_size(val) + 1,
            Event::ReentrancyGuardEvent(val) => Event::cairo_serialized_size(val) + 1,
            _ => 0,
        }
    }
    fn cairo_serialize(
        __rust: &Self::RustType,
    ) -> Vec<starknet::core::types::FieldElement> {
        match __rust {
            Event::AdminPropertiesSet(val) => {
                let mut temp = vec![];
                temp.extend(usize::cairo_serialize(&0usize));
                temp.extend(AdminPropertiesSet::cairo_serialize(val));
                temp
            }
            Event::AssertionSettled(val) => {
                let mut temp = vec![];
                temp.extend(usize::cairo_serialize(&1usize));
                temp.extend(AssertionSettled::cairo_serialize(val));
                temp
            }
            Event::AssertionMade(val) => {
                let mut temp = vec![];
                temp.extend(usize::cairo_serialize(&2usize));
                temp.extend(AssertionMade::cairo_serialize(val));
                temp
            }
            Event::AssertionDisputed(val) => {
                let mut temp = vec![];
                temp.extend(usize::cairo_serialize(&3usize));
                temp.extend(AssertionDisputed::cairo_serialize(val));
                temp
            }
            Event::OwnableEvent(val) => {
                let mut temp = vec![];
                temp.extend(usize::cairo_serialize(&4usize));
                temp.extend(OwnableCptEvent::cairo_serialize(val));
                temp
            }
            Event::ReentrancyGuardEvent(val) => {
                let mut temp = vec![];
                temp.extend(usize::cairo_serialize(&5usize));
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
                    Event::AdminPropertiesSet(
                        AdminPropertiesSet::cairo_deserialize(__felts, __offset + 1)?,
                    ),
                )
            }
            1usize => {
                Ok(
                    Event::AssertionSettled(
                        AssertionSettled::cairo_deserialize(__felts, __offset + 1)?,
                    ),
                )
            }
            2usize => {
                Ok(
                    Event::AssertionMade(
                        AssertionMade::cairo_deserialize(__felts, __offset + 1)?,
                    ),
                )
            }
            3usize => {
                Ok(
                    Event::AssertionDisputed(
                        AssertionDisputed::cairo_deserialize(__felts, __offset + 1)?,
                    ),
                )
            }
            4usize => {
                Ok(
                    Event::OwnableEvent(
                        OwnableCptEvent::cairo_deserialize(__felts, __offset + 1)?,
                    ),
                )
            }
            5usize => {
                Ok(
                    Event::ReentrancyGuardEvent(
                        Event::cairo_deserialize(__felts, __offset + 1)?,
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
            == starknet::core::utils::get_selector_from_name("AdminPropertiesSet")
                .unwrap_or_else(|_| {
                    panic!("Invalid selector for {}", "AdminPropertiesSet")
                })
        {
            let mut key_offset = 0 + 1;
            let mut data_offset = 0;
            let default_currency = match cainome::cairo_serde::ContractAddress::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(
                        format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "default_currency",
                            "AdminPropertiesSet",
                            e,
                        ),
                    );
                }
            };
            data_offset
                += cainome::cairo_serde::ContractAddress::cairo_serialized_size(
                    &default_currency,
                );
            let default_liveness = match u64::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(
                        format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "default_liveness",
                            "AdminPropertiesSet",
                            e,
                        ),
                    );
                }
            };
            data_offset += u64::cairo_serialized_size(&default_liveness);
            let burned_bond_percentage = match cainome::cairo_serde::U256::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(
                        format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "burned_bond_percentage",
                            "AdminPropertiesSet",
                            e,
                        ),
                    );
                }
            };
            data_offset
                += cainome::cairo_serde::U256::cairo_serialized_size(
                    &burned_bond_percentage,
                );
            return Ok(
                Event::AdminPropertiesSet(AdminPropertiesSet {
                    default_currency,
                    default_liveness,
                    burned_bond_percentage,
                }),
            );
        }
        let selector = event.keys[0];
        if selector
            == starknet::core::utils::get_selector_from_name("AssertionSettled")
                .unwrap_or_else(|_| {
                    panic!("Invalid selector for {}", "AssertionSettled")
                })
        {
            let mut key_offset = 0 + 1;
            let mut data_offset = 0;
            let assertion_id = match starknet::core::types::FieldElement::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(
                        format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "assertion_id",
                            "AssertionSettled",
                            e,
                        ),
                    );
                }
            };
            data_offset
                += starknet::core::types::FieldElement::cairo_serialized_size(
                    &assertion_id,
                );
            let bond_recipient = match cainome::cairo_serde::ContractAddress::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(
                        format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "bond_recipient",
                            "AssertionSettled",
                            e,
                        ),
                    );
                }
            };
            data_offset
                += cainome::cairo_serde::ContractAddress::cairo_serialized_size(
                    &bond_recipient,
                );
            let disputed = match bool::cairo_deserialize(&event.data, data_offset) {
                Ok(v) => v,
                Err(e) => {
                    return Err(
                        format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "disputed",
                            "AssertionSettled",
                            e,
                        ),
                    );
                }
            };
            data_offset += bool::cairo_serialized_size(&disputed);
            let settlement_resolution = match bool::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(
                        format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "settlement_resolution",
                            "AssertionSettled",
                            e,
                        ),
                    );
                }
            };
            data_offset += bool::cairo_serialized_size(&settlement_resolution);
            let settle_caller = match cainome::cairo_serde::ContractAddress::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(
                        format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "settle_caller",
                            "AssertionSettled",
                            e,
                        ),
                    );
                }
            };
            data_offset
                += cainome::cairo_serde::ContractAddress::cairo_serialized_size(
                    &settle_caller,
                );
            return Ok(
                Event::AssertionSettled(AssertionSettled {
                    assertion_id,
                    bond_recipient,
                    disputed,
                    settlement_resolution,
                    settle_caller,
                }),
            );
        }
        let selector = event.keys[0];
        if selector
            == starknet::core::utils::get_selector_from_name("AssertionMade")
                .unwrap_or_else(|_| panic!("Invalid selector for {}", "AssertionMade"))
        {
            let mut key_offset = 0 + 1;
            let mut data_offset = 0;
            let assertion_id = match starknet::core::types::FieldElement::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(
                        format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "assertion_id",
                            "AssertionMade",
                            e,
                        ),
                    );
                }
            };
            data_offset
                += starknet::core::types::FieldElement::cairo_serialized_size(
                    &assertion_id,
                );
            let domain_id = match cainome::cairo_serde::U256::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(
                        format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "domain_id",
                            "AssertionMade",
                            e,
                        ),
                    );
                }
            };
            data_offset += cainome::cairo_serde::U256::cairo_serialized_size(&domain_id);
            let claim = match cainome::cairo_serde::ByteArray::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(
                        format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "claim",
                            "AssertionMade",
                            e,
                        ),
                    );
                }
            };
            data_offset
                += cainome::cairo_serde::ByteArray::cairo_serialized_size(&claim);
            let asserter = match cainome::cairo_serde::ContractAddress::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(
                        format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "asserter",
                            "AssertionMade",
                            e,
                        ),
                    );
                }
            };
            data_offset
                += cainome::cairo_serde::ContractAddress::cairo_serialized_size(
                    &asserter,
                );
            let callback_recipient = match cainome::cairo_serde::ContractAddress::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(
                        format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "callback_recipient",
                            "AssertionMade",
                            e,
                        ),
                    );
                }
            };
            data_offset
                += cainome::cairo_serde::ContractAddress::cairo_serialized_size(
                    &callback_recipient,
                );
            let escalation_manager = match cainome::cairo_serde::ContractAddress::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(
                        format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "escalation_manager",
                            "AssertionMade",
                            e,
                        ),
                    );
                }
            };
            data_offset
                += cainome::cairo_serde::ContractAddress::cairo_serialized_size(
                    &escalation_manager,
                );
            let caller = match cainome::cairo_serde::ContractAddress::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(
                        format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "caller",
                            "AssertionMade",
                            e,
                        ),
                    );
                }
            };
            data_offset
                += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&caller);
            let expiration_timestamp = match u64::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(
                        format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "expiration_timestamp",
                            "AssertionMade",
                            e,
                        ),
                    );
                }
            };
            data_offset += u64::cairo_serialized_size(&expiration_timestamp);
            let currency = match ERC20ABIDispatcher::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(
                        format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "currency",
                            "AssertionMade",
                            e,
                        ),
                    );
                }
            };
            data_offset += ERC20ABIDispatcher::cairo_serialized_size(&currency);
            let bond = match cainome::cairo_serde::U256::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(
                        format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "bond",
                            "AssertionMade",
                            e,
                        ),
                    );
                }
            };
            data_offset += cainome::cairo_serde::U256::cairo_serialized_size(&bond);
            let identifier = match starknet::core::types::FieldElement::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(
                        format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "identifier",
                            "AssertionMade",
                            e,
                        ),
                    );
                }
            };
            data_offset
                += starknet::core::types::FieldElement::cairo_serialized_size(
                    &identifier,
                );
            return Ok(
                Event::AssertionMade(AssertionMade {
                    assertion_id,
                    domain_id,
                    claim,
                    asserter,
                    callback_recipient,
                    escalation_manager,
                    caller,
                    expiration_timestamp,
                    currency,
                    bond,
                    identifier,
                }),
            );
        }
        let selector = event.keys[0];
        if selector
            == starknet::core::utils::get_selector_from_name("AssertionDisputed")
                .unwrap_or_else(|_| {
                    panic!("Invalid selector for {}", "AssertionDisputed")
                })
        {
            let mut key_offset = 0 + 1;
            let mut data_offset = 0;
            let assertion_id = match starknet::core::types::FieldElement::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(
                        format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "assertion_id",
                            "AssertionDisputed",
                            e,
                        ),
                    );
                }
            };
            data_offset
                += starknet::core::types::FieldElement::cairo_serialized_size(
                    &assertion_id,
                );
            let caller = match cainome::cairo_serde::ContractAddress::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(
                        format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "caller",
                            "AssertionDisputed",
                            e,
                        ),
                    );
                }
            };
            data_offset
                += cainome::cairo_serde::ContractAddress::cairo_serialized_size(&caller);
            let disputer = match cainome::cairo_serde::ContractAddress::cairo_deserialize(
                &event.data,
                data_offset,
            ) {
                Ok(v) => v,
                Err(e) => {
                    return Err(
                        format!(
                            "Could not deserialize field {} for {}: {:?}",
                            "disputer",
                            "AssertionDisputed",
                            e,
                        ),
                    );
                }
            };
            data_offset
                += cainome::cairo_serde::ContractAddress::cairo_serialized_size(
                    &disputer,
                );
            return Ok(
                Event::AssertionDisputed(AssertionDisputed {
                    assertion_id,
                    caller,
                    disputer,
                }),
            );
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
                    OwnableCptEvent::OwnershipTransferred(OwnershipTransferred {
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
                    OwnableCptEvent::OwnershipTransferStarted(OwnershipTransferStarted {
                        previous_owner,
                        new_owner,
                    }),
                ),
            );
        }
        Err(format!("Could not match any event from keys {:?}", event.keys))
    }
}
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Event {}
impl cainome::cairo_serde::CairoSerde for Event {
    type RustType = Self;
    const SERIALIZED_SIZE: std::option::Option<usize> = std::option::Option::None;
    #[inline]
    fn cairo_serialized_size(__rust: &Self::RustType) -> usize {
        match __rust {
            _ => 0,
        }
    }
    fn cairo_serialize(
        __rust: &Self::RustType,
    ) -> Vec<starknet::core::types::FieldElement> {
        match __rust {
            _ => vec![],
        }
    }
    fn cairo_deserialize(
        __felts: &[starknet::core::types::FieldElement],
        __offset: usize,
    ) -> cainome::cairo_serde::Result<Self::RustType> {
        let __index: u128 = __felts[__offset].try_into().unwrap();
        match __index as usize {
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
        Err(format!("Could not match any event from keys {:?}", event.keys))
    }
}
impl<A: starknet::accounts::ConnectedAccount + Sync> optimistic_oracle_v1<A> {
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn get_minimum_bond(
        &self,
        currency: &cainome::cairo_serde::ContractAddress,
    ) -> cainome::cairo_serde::call::FCall<A::Provider, cainome::cairo_serde::U256> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata
            .extend(cainome::cairo_serde::ContractAddress::cairo_serialize(currency));
        let __call = starknet::core::types::FunctionCall {
            contract_address: self.address,
            entry_point_selector: starknet::macros::selector!("get_minimum_bond"),
            calldata: __calldata,
        };
        cainome::cairo_serde::call::FCall::new(__call, self.provider())
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn stamp_assertion(
        &self,
        assertion_id: &starknet::core::types::FieldElement,
    ) -> cainome::cairo_serde::call::FCall<
        A::Provider,
        cainome::cairo_serde::ByteArray,
    > {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata
            .extend(starknet::core::types::FieldElement::cairo_serialize(assertion_id));
        let __call = starknet::core::types::FunctionCall {
            contract_address: self.address,
            entry_point_selector: starknet::macros::selector!("stamp_assertion"),
            calldata: __calldata,
        };
        cainome::cairo_serde::call::FCall::new(__call, self.provider())
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn default_identifier(
        &self,
    ) -> cainome::cairo_serde::call::FCall<
        A::Provider,
        starknet::core::types::FieldElement,
    > {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        let __call = starknet::core::types::FunctionCall {
            contract_address: self.address,
            entry_point_selector: starknet::macros::selector!("default_identifier"),
            calldata: __calldata,
        };
        cainome::cairo_serde::call::FCall::new(__call, self.provider())
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn get_assertion(
        &self,
        assertion_id: &starknet::core::types::FieldElement,
    ) -> cainome::cairo_serde::call::FCall<A::Provider, Assertion> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata
            .extend(starknet::core::types::FieldElement::cairo_serialize(assertion_id));
        let __call = starknet::core::types::FunctionCall {
            contract_address: self.address,
            entry_point_selector: starknet::macros::selector!("get_assertion"),
            calldata: __calldata,
        };
        cainome::cairo_serde::call::FCall::new(__call, self.provider())
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn get_assertion_result(
        &self,
        assertion_id: &starknet::core::types::FieldElement,
    ) -> cainome::cairo_serde::call::FCall<A::Provider, bool> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata
            .extend(starknet::core::types::FieldElement::cairo_serialize(assertion_id));
        let __call = starknet::core::types::FunctionCall {
            contract_address: self.address,
            entry_point_selector: starknet::macros::selector!("get_assertion_result"),
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
    pub fn assert_truth_with_defaults_getcall(
        &self,
        claim: &cainome::cairo_serde::ByteArray,
        asserter: &cainome::cairo_serde::ContractAddress,
    ) -> starknet::accounts::Call {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(cainome::cairo_serde::ByteArray::cairo_serialize(claim));
        __calldata
            .extend(cainome::cairo_serde::ContractAddress::cairo_serialize(asserter));
        starknet::accounts::Call {
            to: self.address,
            selector: starknet::macros::selector!("assert_truth_with_defaults"),
            calldata: __calldata,
        }
    }
    #[allow(clippy::ptr_arg)]
    pub fn assert_truth_with_defaults(
        &self,
        claim: &cainome::cairo_serde::ByteArray,
        asserter: &cainome::cairo_serde::ContractAddress,
    ) -> starknet::accounts::Execution<A> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(cainome::cairo_serde::ByteArray::cairo_serialize(claim));
        __calldata
            .extend(cainome::cairo_serde::ContractAddress::cairo_serialize(asserter));
        let __call = starknet::accounts::Call {
            to: self.address,
            selector: starknet::macros::selector!("assert_truth_with_defaults"),
            calldata: __calldata,
        };
        self.account.execute(vec![__call])
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn assert_truth_getcall(
        &self,
        claim: &cainome::cairo_serde::ByteArray,
        asserter: &cainome::cairo_serde::ContractAddress,
        callback_recipient: &cainome::cairo_serde::ContractAddress,
        escalation_manager: &cainome::cairo_serde::ContractAddress,
        liveness: &u64,
        currency: &ERC20ABIDispatcher,
        bond: &cainome::cairo_serde::U256,
        identifier: &starknet::core::types::FieldElement,
        domain_id: &cainome::cairo_serde::U256,
    ) -> starknet::accounts::Call {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(cainome::cairo_serde::ByteArray::cairo_serialize(claim));
        __calldata
            .extend(cainome::cairo_serde::ContractAddress::cairo_serialize(asserter));
        __calldata
            .extend(
                cainome::cairo_serde::ContractAddress::cairo_serialize(
                    callback_recipient,
                ),
            );
        __calldata
            .extend(
                cainome::cairo_serde::ContractAddress::cairo_serialize(
                    escalation_manager,
                ),
            );
        __calldata.extend(u64::cairo_serialize(liveness));
        __calldata.extend(ERC20ABIDispatcher::cairo_serialize(currency));
        __calldata.extend(cainome::cairo_serde::U256::cairo_serialize(bond));
        __calldata
            .extend(starknet::core::types::FieldElement::cairo_serialize(identifier));
        __calldata.extend(cainome::cairo_serde::U256::cairo_serialize(domain_id));
        starknet::accounts::Call {
            to: self.address,
            selector: starknet::macros::selector!("assert_truth"),
            calldata: __calldata,
        }
    }
    #[allow(clippy::ptr_arg)]
    pub fn assert_truth(
        &self,
        claim: &cainome::cairo_serde::ByteArray,
        asserter: &cainome::cairo_serde::ContractAddress,
        callback_recipient: &cainome::cairo_serde::ContractAddress,
        escalation_manager: &cainome::cairo_serde::ContractAddress,
        liveness: &u64,
        currency: &ERC20ABIDispatcher,
        bond: &cainome::cairo_serde::U256,
        identifier: &starknet::core::types::FieldElement,
        domain_id: &cainome::cairo_serde::U256,
    ) -> starknet::accounts::Execution<A> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata.extend(cainome::cairo_serde::ByteArray::cairo_serialize(claim));
        __calldata
            .extend(cainome::cairo_serde::ContractAddress::cairo_serialize(asserter));
        __calldata
            .extend(
                cainome::cairo_serde::ContractAddress::cairo_serialize(
                    callback_recipient,
                ),
            );
        __calldata
            .extend(
                cainome::cairo_serde::ContractAddress::cairo_serialize(
                    escalation_manager,
                ),
            );
        __calldata.extend(u64::cairo_serialize(liveness));
        __calldata.extend(ERC20ABIDispatcher::cairo_serialize(currency));
        __calldata.extend(cainome::cairo_serde::U256::cairo_serialize(bond));
        __calldata
            .extend(starknet::core::types::FieldElement::cairo_serialize(identifier));
        __calldata.extend(cainome::cairo_serde::U256::cairo_serialize(domain_id));
        let __call = starknet::accounts::Call {
            to: self.address,
            selector: starknet::macros::selector!("assert_truth"),
            calldata: __calldata,
        };
        self.account.execute(vec![__call])
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn dispute_assertion_getcall(
        &self,
        assertion_id: &starknet::core::types::FieldElement,
        disputer: &cainome::cairo_serde::ContractAddress,
    ) -> starknet::accounts::Call {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata
            .extend(starknet::core::types::FieldElement::cairo_serialize(assertion_id));
        __calldata
            .extend(cainome::cairo_serde::ContractAddress::cairo_serialize(disputer));
        starknet::accounts::Call {
            to: self.address,
            selector: starknet::macros::selector!("dispute_assertion"),
            calldata: __calldata,
        }
    }
    #[allow(clippy::ptr_arg)]
    pub fn dispute_assertion(
        &self,
        assertion_id: &starknet::core::types::FieldElement,
        disputer: &cainome::cairo_serde::ContractAddress,
    ) -> starknet::accounts::Execution<A> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata
            .extend(starknet::core::types::FieldElement::cairo_serialize(assertion_id));
        __calldata
            .extend(cainome::cairo_serde::ContractAddress::cairo_serialize(disputer));
        let __call = starknet::accounts::Call {
            to: self.address,
            selector: starknet::macros::selector!("dispute_assertion"),
            calldata: __calldata,
        };
        self.account.execute(vec![__call])
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn settle_assertion_getcall(
        &self,
        assertion_id: &starknet::core::types::FieldElement,
    ) -> starknet::accounts::Call {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata
            .extend(starknet::core::types::FieldElement::cairo_serialize(assertion_id));
        starknet::accounts::Call {
            to: self.address,
            selector: starknet::macros::selector!("settle_assertion"),
            calldata: __calldata,
        }
    }
    #[allow(clippy::ptr_arg)]
    pub fn settle_assertion(
        &self,
        assertion_id: &starknet::core::types::FieldElement,
    ) -> starknet::accounts::Execution<A> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata
            .extend(starknet::core::types::FieldElement::cairo_serialize(assertion_id));
        let __call = starknet::accounts::Call {
            to: self.address,
            selector: starknet::macros::selector!("settle_assertion"),
            calldata: __calldata,
        };
        self.account.execute(vec![__call])
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn sync_params_getcall(
        &self,
        identifier: &starknet::core::types::FieldElement,
        currency: &cainome::cairo_serde::ContractAddress,
    ) -> starknet::accounts::Call {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata
            .extend(starknet::core::types::FieldElement::cairo_serialize(identifier));
        __calldata
            .extend(cainome::cairo_serde::ContractAddress::cairo_serialize(currency));
        starknet::accounts::Call {
            to: self.address,
            selector: starknet::macros::selector!("sync_params"),
            calldata: __calldata,
        }
    }
    #[allow(clippy::ptr_arg)]
    pub fn sync_params(
        &self,
        identifier: &starknet::core::types::FieldElement,
        currency: &cainome::cairo_serde::ContractAddress,
    ) -> starknet::accounts::Execution<A> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata
            .extend(starknet::core::types::FieldElement::cairo_serialize(identifier));
        __calldata
            .extend(cainome::cairo_serde::ContractAddress::cairo_serialize(currency));
        let __call = starknet::accounts::Call {
            to: self.address,
            selector: starknet::macros::selector!("sync_params"),
            calldata: __calldata,
        };
        self.account.execute(vec![__call])
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn settle_and_get_assertion_result_getcall(
        &self,
        assertion_id: &starknet::core::types::FieldElement,
    ) -> starknet::accounts::Call {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata
            .extend(starknet::core::types::FieldElement::cairo_serialize(assertion_id));
        starknet::accounts::Call {
            to: self.address,
            selector: starknet::macros::selector!("settle_and_get_assertion_result"),
            calldata: __calldata,
        }
    }
    #[allow(clippy::ptr_arg)]
    pub fn settle_and_get_assertion_result(
        &self,
        assertion_id: &starknet::core::types::FieldElement,
    ) -> starknet::accounts::Execution<A> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata
            .extend(starknet::core::types::FieldElement::cairo_serialize(assertion_id));
        let __call = starknet::accounts::Call {
            to: self.address,
            selector: starknet::macros::selector!("settle_and_get_assertion_result"),
            calldata: __calldata,
        };
        self.account.execute(vec![__call])
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn set_admin_properties_getcall(
        &self,
        default_currency: &cainome::cairo_serde::ContractAddress,
        default_liveness: &u64,
        burned_bond_percentage: &cainome::cairo_serde::U256,
    ) -> starknet::accounts::Call {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata
            .extend(
                cainome::cairo_serde::ContractAddress::cairo_serialize(default_currency),
            );
        __calldata.extend(u64::cairo_serialize(default_liveness));
        __calldata
            .extend(cainome::cairo_serde::U256::cairo_serialize(burned_bond_percentage));
        starknet::accounts::Call {
            to: self.address,
            selector: starknet::macros::selector!("set_admin_properties"),
            calldata: __calldata,
        }
    }
    #[allow(clippy::ptr_arg)]
    pub fn set_admin_properties(
        &self,
        default_currency: &cainome::cairo_serde::ContractAddress,
        default_liveness: &u64,
        burned_bond_percentage: &cainome::cairo_serde::U256,
    ) -> starknet::accounts::Execution<A> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata
            .extend(
                cainome::cairo_serde::ContractAddress::cairo_serialize(default_currency),
            );
        __calldata.extend(u64::cairo_serialize(default_liveness));
        __calldata
            .extend(cainome::cairo_serde::U256::cairo_serialize(burned_bond_percentage));
        let __call = starknet::accounts::Call {
            to: self.address,
            selector: starknet::macros::selector!("set_admin_properties"),
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
impl<P: starknet::providers::Provider + Sync> optimistic_oracle_v1Reader<P> {
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn get_minimum_bond(
        &self,
        currency: &cainome::cairo_serde::ContractAddress,
    ) -> cainome::cairo_serde::call::FCall<P, cainome::cairo_serde::U256> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata
            .extend(cainome::cairo_serde::ContractAddress::cairo_serialize(currency));
        let __call = starknet::core::types::FunctionCall {
            contract_address: self.address,
            entry_point_selector: starknet::macros::selector!("get_minimum_bond"),
            calldata: __calldata,
        };
        cainome::cairo_serde::call::FCall::new(__call, self.provider())
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn stamp_assertion(
        &self,
        assertion_id: &starknet::core::types::FieldElement,
    ) -> cainome::cairo_serde::call::FCall<P, cainome::cairo_serde::ByteArray> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata
            .extend(starknet::core::types::FieldElement::cairo_serialize(assertion_id));
        let __call = starknet::core::types::FunctionCall {
            contract_address: self.address,
            entry_point_selector: starknet::macros::selector!("stamp_assertion"),
            calldata: __calldata,
        };
        cainome::cairo_serde::call::FCall::new(__call, self.provider())
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn default_identifier(
        &self,
    ) -> cainome::cairo_serde::call::FCall<P, starknet::core::types::FieldElement> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        let __call = starknet::core::types::FunctionCall {
            contract_address: self.address,
            entry_point_selector: starknet::macros::selector!("default_identifier"),
            calldata: __calldata,
        };
        cainome::cairo_serde::call::FCall::new(__call, self.provider())
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn get_assertion(
        &self,
        assertion_id: &starknet::core::types::FieldElement,
    ) -> cainome::cairo_serde::call::FCall<P, Assertion> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata
            .extend(starknet::core::types::FieldElement::cairo_serialize(assertion_id));
        let __call = starknet::core::types::FunctionCall {
            contract_address: self.address,
            entry_point_selector: starknet::macros::selector!("get_assertion"),
            calldata: __calldata,
        };
        cainome::cairo_serde::call::FCall::new(__call, self.provider())
    }
    #[allow(clippy::ptr_arg)]
    #[allow(clippy::too_many_arguments)]
    pub fn get_assertion_result(
        &self,
        assertion_id: &starknet::core::types::FieldElement,
    ) -> cainome::cairo_serde::call::FCall<P, bool> {
        use cainome::cairo_serde::CairoSerde;
        let mut __calldata = vec![];
        __calldata
            .extend(starknet::core::types::FieldElement::cairo_serialize(assertion_id));
        let __call = starknet::core::types::FunctionCall {
            contract_address: self.address,
            entry_point_selector: starknet::macros::selector!("get_assertion_result"),
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
