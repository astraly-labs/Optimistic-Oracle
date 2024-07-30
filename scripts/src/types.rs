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

pub struct OO_Config {
    pub liveness: u64,
    pub erc20_token: FieldElement,
    pub final_fee: cainome::cairo_serde::U256,
}

pub mod oracle_interfaces {
    use super::{felt, FieldElement};
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
                OracleInterface::ORACLE => felt!("0x0x4f7261636c65"),
                OracleInterface::IDENTIFIER_WHITELIST => {
                    felt!("0x4964656e74696669657257686974656c697374")
                }
                OracleInterface::COLLATERAL_WHITELIST => {
                    felt!("0x436f6c6c61746572616c57686974656c697374")
                }
                OracleInterface::STORE => felt!("0x53746f7265"),
            }
        }
    }
}
