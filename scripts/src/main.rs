use dotenv::dotenv;
use serde_json;
use starknet::{
    accounts::{Account, Call, ExecutionEncoding, SingleOwnerAccount},
    contract::ContractFactory,
    core::{
        chain_id,
        types::{contract::CompiledClass, FieldElement, FlattenedSierraClass},
    },
    macros::felt,
    providers::{jsonrpc::HttpTransport, AnyProvider, JsonRpcClient, Provider, Url},
    signers::{LocalWallet, SigningKey},
};
use std::env;
use std::sync::Arc;
use types::OO_Config;
use types::{Codes, FormattedCodes};
mod types;
mod utils;
use std::fs::File;
use std::io::Write;
mod deploy;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    dotenv().ok();
    let provider = AnyProvider::JsonRpcHttp(JsonRpcClient::new(HttpTransport::new(
        Url::parse("https://alpha-sepolia.starknet.io/").unwrap(),
    )));

    let private_key =
        env::var("STARKNET_PRIVATE_KEY").expect("STARKNET_PRIVATE_KEY not set in environment");

    let account_address = env::var("STARKNET_ACCOUNT_ADDRESS")
        .expect("STARKNET_ACCOUNT_ADDRESS not set in environment");

    let secret_scalar = FieldElement::from_hex_be(&private_key).unwrap();
    let signing_key = SigningKey::from_secret_scalar(secret_scalar);
    let signer = LocalWallet::from(signing_key);

    // Set up the account (replace with your account address)
    let account_address = FieldElement::from_hex_be(&account_address).unwrap();
    let account = SingleOwnerAccount::new(
        provider,
        signer,
        account_address,
        chain_id::SEPOLIA,
        ExecutionEncoding::New,
    );

    let declarations = utils::declare_all(&account).await.unwrap();

    let oo_config = types::OO_Config {
        liveness: 120,
        erc20_token: types::ETH_ADDRESS,
        final_fee: cainome::cairo_serde::U256 {
            low: 100000000,
            high: 0,
        },
    };
    let deployments = deploy::deploy_core(&account, &account, oo_config, &declarations).await?;

    // Create the formatted structure
    let formatted_codes = FormattedCodes {
        codes: deployments,
        network: "starknet_sepolia".to_string(),
    };

    // Serialize to JSON
    let json = serde_json::to_string_pretty(&formatted_codes)?;

    // Write to file
    let mut file = File::create("sepolia_deployments.json")?;
    file.write_all(json.as_bytes())?;

    println!("JSON file 'starknet_addresses.json' has been created successfully.");

    Ok(())
}
