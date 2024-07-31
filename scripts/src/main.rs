use dotenv::dotenv;
use log::info;
use serde_json;
use starknet::{
    accounts::{ExecutionEncoding, SingleOwnerAccount},
    core::{chain_id, types::FieldElement},
    providers::{jsonrpc::HttpTransport, AnyProvider, JsonRpcClient, Provider, Url},
    signers::{LocalWallet, SigningKey},
};
use std::{env, fs::File, io::Write};
use types::FormattedCodes;

mod build;
mod deploy;
mod types;
mod utils;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    env_logger::init();

    dotenv().ok();
    let provider = AnyProvider::JsonRpcHttp(JsonRpcClient::new(HttpTransport::new(
        Url::parse("https://free-rpc.nethermind.io/sepolia-juno").unwrap(),
    )));

    // Test out the provider
    // Add this after creating the provider
    provider
        .block_number()
        .await
        .map_err(|e| eyre::eyre!("Failed to connect to provider: {}", e))?;

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

    let declarations = utils::declare_all(&account).await?;

    let oo_config = types::OOConfig {
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

    info!("JSON file 'starknet_addresses.json' has been created successfully.");

    Ok(())
}
