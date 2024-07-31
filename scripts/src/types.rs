use cainome::cairo_serde::CairoSerde;
use starknet::{
    accounts::{Account, ConnectedAccount, SingleOwnerAccount},
    contract::ContractFactory,
    core::types::Felt,
    macros::felt,
    providers::{jsonrpc::HttpTransport, AnyProvider, JsonRpcClient, Provider, ProviderError, Url},
    signers::{LocalWallet, SigningKey},
};
use std::future::Future;
use std::sync::Arc;

pub type StarknetAccount = SingleOwnerAccount<AnyProvider, LocalWallet>;
pub const ETH_ADDRESS: Felt =
    felt!("0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7");
pub const ASSERT_TRUTH: Felt = felt!("0x4153534552545f5452555448");
pub const ORACLE: Felt = felt!("0x4f7261636c65");
pub const IDENTIFIER_WHITELIST: Felt = felt!("0x4964656e74696669657257686974656c697374");
pub const COLLATERAL_WHITELIST: Felt = felt!("0x436f6c6c61746572616c57686974656c697374");
pub const STORE: Felt = felt!("0x53746f7265");

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Codes {
    pub finder: Felt,
    pub address_whitelist: Felt,
    pub identifier_whitelist: Felt,
    pub store: Felt,
    pub oracle_ancillary: Felt,
    pub optimistic_oracle_v1: Felt,
}

#[derive(serde::Serialize)]
pub struct FormattedCodes {
    pub codes: Codes,
    pub network: String,
}

pub struct OOConfig {
    pub liveness: u64,
    pub erc20_token: Felt,
    pub final_fee: cainome::cairo_serde::U256,
}

pub struct OODeploymentArguments {
    pub finder: Felt,
    pub erc20: Felt,
    pub liveness: u64,
}

pub struct LibraryContracts {
    pub finder: Felt,
    pub store: Felt,
    pub identifier_whitelist: Felt,
    pub address_whitelist: Felt,
    pub oracle: Felt,
}

pub mod oracle_interfaces {
    use super::{felt, Felt, COLLATERAL_WHITELIST, IDENTIFIER_WHITELIST, ORACLE, STORE};
    #[derive(Debug)]
    pub enum OracleInterface {
        ORACLE,
        IDENTIFIER_WHITELIST,
        COLLATERAL_WHITELIST,
        STORE,
    }

    impl OracleInterface {
        pub fn as_str(&self) -> Felt {
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
