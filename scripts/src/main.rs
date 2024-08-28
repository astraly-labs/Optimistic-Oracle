use dotenv::dotenv;
use log::info;
use serde_json;
use starknet::{
    accounts::{ExecutionEncoding, SingleOwnerAccount},
    core::{
        chain_id,
        types::{BlockId, BlockTag, Felt},
    },
    providers::{jsonrpc::HttpTransport, AnyProvider, JsonRpcClient, Provider, Url},
    signers::{LocalWallet, SigningKey},
};

use std::{env, fs::File, io::Write};

use scripts::types::FormattedCodes;

const PROVIDER_URL: &str = "https://starknet-sepolia.public.blastapi.io/rpc/v0_7";
#[tokio::main]
async fn main() -> eyre::Result<()> {
    env_logger::init();

    dotenv().ok();
    let provider = AnyProvider::JsonRpcHttp(JsonRpcClient::new(HttpTransport::new(
        Url::parse(PROVIDER_URL).unwrap(),
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

    let secret_scalar = Felt::from_hex(&private_key).unwrap();
    let signing_key = SigningKey::from_secret_scalar(secret_scalar);
    let signer = LocalWallet::from(signing_key);

    // Set up the account (replace with your account address)
    let account_address = Felt::from_hex(&account_address).unwrap();
    let mut account = SingleOwnerAccount::new(
        provider,
        signer,
        account_address,
        chain_id::SEPOLIA,
        ExecutionEncoding::New,
    );

    account.set_block_id(BlockId::Tag(BlockTag::Pending));

    let declarations = scripts::utils::declare_all(&account).await?;

    let oo_config = scripts::types::OOConfig {
        liveness: 7200,
        erc20_token: scripts::types::ETH_ADDRESS,
        final_fee: cainome::cairo_serde::U256 {
            low: 100000000,
            high: 0,
        },
    };
    let deployments =
        scripts::deploy::deploy_core(&account, &account, oo_config, &declarations).await?;

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
