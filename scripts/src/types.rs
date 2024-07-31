use cainome::cairo_serde::CairoSerde;
use starknet::{
    accounts::{Account, ConnectedAccount, SingleOwnerAccount},
    contract::ContractFactory,
    core::types::{
        contract::{CompiledClass, SierraClass},
        BlockId, BlockTag, ExecutionResult, FieldElement, FlattenedSierraClass,
        InvokeTransactionResult, MaybePendingTransactionReceipt, StarknetError,
    },
    macros::felt,
    providers::{jsonrpc::HttpTransport, AnyProvider, JsonRpcClient, Provider, ProviderError, Url},
    signers::{LocalWallet, SigningKey},
};
use std::future::Future;
use std::sync::Arc;

pub type StarknetAccount = SingleOwnerAccount<AnyProvider, LocalWallet>;
pub const ETH_ADDRESS: FieldElement =
    felt!("0x0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7");
pub const ASSERT_TRUTH: FieldElement = felt!("0x4153534552545f5452555448");
pub const ORACLE: FieldElement = felt!("0x0x4f7261636c65");
pub const IDENTIFIER_WHITELIST: FieldElement = felt!("0x4964656e74696669657257686974656c697374");
pub const COLLATERAL_WHITELIST: FieldElement = felt!("0x436f6c6c61746572616c57686974656c697374");
pub const STORE: FieldElement = felt!("0x53746f7265");

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Codes {
    pub finder: FieldElement,
    pub address_whitelist: FieldElement,
    pub identifier_whitelist: FieldElement,
    pub store: FieldElement,
    pub oracle_ancillary: FieldElement,
    pub optimistic_oracle_v1: FieldElement,
}

#[derive(serde::Serialize)]
pub struct FormattedCodes {
    pub codes: Codes,
    pub network: String,
}

pub struct OOConfig {
    pub liveness: u64,
    pub erc20_token: FieldElement,
    pub final_fee: cainome::cairo_serde::U256,
}

pub struct OODeploymentArguments {
    pub finder: FieldElement,
    pub erc20: FieldElement,
    pub liveness: u64,
}

pub struct LibraryContracts {
    pub finder: FieldElement,
    pub store: FieldElement,
    pub identifier_whitelist: FieldElement,
    pub address_whitelist: FieldElement,
    pub oracle: FieldElement,
}

pub mod oracle_interfaces {
    use super::{felt, FieldElement, COLLATERAL_WHITELIST, IDENTIFIER_WHITELIST, ORACLE, STORE};
    #[derive(Debug)]
    pub enum OracleInterface {
        ORACLE,
        IDENTIFIER_WHITELIST,
        COLLATERAL_WHITELIST,
        STORE,
    }

    impl OracleInterface {
        pub fn as_str(&self) -> FieldElement {
            match self {
                // TODO(best practice): find the function that does the conversion directly String -> felt
                OracleInterface::ORACLE => ORACLE,
                OracleInterface::IDENTIFIER_WHITELIST => IDENTIFIER_WHITELIST,
                OracleInterface::COLLATERAL_WHITELIST => COLLATERAL_WHITELIST,
                OracleInterface::STORE => STORE,
            }
        }
    }
}
