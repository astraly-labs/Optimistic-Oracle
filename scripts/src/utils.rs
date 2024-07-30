use starknet::{
    accounts::{Account, Call, ConnectedAccount, SingleOwnerAccount},
    contract::ContractFactory,
    core::types::{
        contract::{CompiledClass, SierraClass},
        BlockId, BlockTag, ExecutionResult, FieldElement, FlattenedSierraClass,
        InvokeTransactionResult, MaybePendingTransactionReceipt, StarknetError,
    },
    core::utils::get_selector_from_name,
    macros::felt,
    providers::{jsonrpc::HttpTransport, AnyProvider, JsonRpcClient, Provider, ProviderError, Url},
    signers::{LocalWallet, SigningKey},
};
use std::future::Future;
use std::sync::Arc;

use super::types::{Codes, StarknetAccount};

const BUILD_PATH_PREFIX: &str = "../optimistic_oracle/target/dev/optimistic_oracle_";

/// Deploys a contract with the given class hash, constructor calldata, and salt.
/// Returns the deployed address and the transaction result.
pub async fn deploy_contract(
    class_hash: FieldElement,
    constructor_calldata: Vec<FieldElement>,
    deployer: &StarknetAccount,
) -> (FieldElement, InvokeTransactionResult) {
    let contract_factory = ContractFactory::new(class_hash, deployer);
    let salt = felt!("0");

    let deployment = contract_factory.deploy(constructor_calldata, salt, false);

    tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
    let deploy_res = deployment.send().await.expect("Failed to deploy contract");

    let receipt = get_transaction_receipt(deployer.provider(), deploy_res.transaction_hash)
        .await
        .expect("Failed to get transaction receipt");

    match receipt.execution_result() {
        ExecutionResult::Reverted { reason } => {
            panic!("Deployment reverted: {}", reason)
        }
        _ => {}
    }

    (deployment.deployed_address(), deploy_res)
}

/// Get the contract artifact from the build directory.
/// # Arguments
/// * `path` - The path to the contract artifact.
/// # Returns
/// The contract artifact.
fn contract_artifacts(contract_name: &str) -> eyre::Result<(FlattenedSierraClass, FieldElement)> {
    let artifact_path = format!("{BUILD_PATH_PREFIX}{contract_name}.contract_class.json");
    let file = std::fs::File::open(artifact_path.clone())?;
    let sierra_class: SierraClass = serde_json::from_reader(file)?;

    let artifact_path = format!("{BUILD_PATH_PREFIX}{contract_name}.compiled_contract_class.json");
    let file = std::fs::File::open(artifact_path)?;

    let compiled_class: CompiledClass = serde_json::from_reader(file)?;

    Ok((sierra_class.flatten()?, compiled_class.class_hash()?))
}

/// Declare a contract class. If the contract class is already declared, do nothing.
/// # Arguments
/// * `account` - The StarkNet account.
/// * `contract_name` - The contract name.
/// # Returns
/// The contract class hash.
async fn declare_contract(
    account: &StarknetAccount,
    contract_name: &str,
) -> eyre::Result<FieldElement> {
    // Load the contract artifact.
    let (flattened_class, compiled_class_hash) = contract_artifacts(contract_name)?;
    let class_hash = flattened_class.class_hash();

    // Declare the contract class if it is not already declared.
    if !is_already_declared(account.provider(), &class_hash).await? {
        println!("\n==> Declaring Contract: {contract_name}");
        account
            .declare(Arc::new(flattened_class), compiled_class_hash)
            .send()
            .await?;
        println!("Declared Class Hash: {}", format!("{:#064x}", class_hash));
    };

    Ok(class_hash)
}

pub async fn assert_poll<F, Fut>(f: F, polling_time_ms: u64, max_poll_count: u32)
where
    F: Fn() -> Fut,
    Fut: Future<Output = bool>,
{
    for _poll_count in 0..max_poll_count {
        if f().await {
            return; // The provided function returned true, exit safely.
        }

        tokio::time::sleep(tokio::time::Duration::from_millis(polling_time_ms)).await;
    }

    panic!("Max poll count exceeded.");
}

type TransactionReceiptResult = Result<MaybePendingTransactionReceipt, ProviderError>;

pub async fn get_transaction_receipt(
    rpc: &AnyProvider,
    transaction_hash: FieldElement,
) -> TransactionReceiptResult {
    // there is a delay between the transaction being available at the client
    // and the sealing of the block, hence sleeping for 100ms
    assert_poll(
        || async { rpc.get_transaction_receipt(transaction_hash).await.is_ok() },
        100,
        20,
    )
    .await;

    rpc.get_transaction_receipt(transaction_hash).await
}

/// Check if a contract class is already declared.
/// # Arguments
/// * `provider` - The StarkNet provider.
/// * `class_hash` - The contract class hash.
/// # Returns
/// `true` if the contract class is already declared, `false` otherwise.
async fn is_already_declared<P>(provider: &P, class_hash: &FieldElement) -> eyre::Result<bool>
where
    P: Provider,
{
    match provider
        .get_class(BlockId::Tag(BlockTag::Pending), class_hash)
        .await
    {
        Ok(_) => {
            eprintln!("Not declaring class as it's already declared. Class hash:");
            println!("{}", format!("{:#064x}", class_hash));

            Ok(true)
        }
        Err(ProviderError::StarknetError(StarknetError::ClassHashNotFound)) => Ok(false),
        Err(err) => Err(err.into()),
    }
}

pub async fn declare_all(deployer: &StarknetAccount) -> eyre::Result<Codes> {
    let finder = declare_contract(deployer, "finder").await?;
    let address_whitelist = declare_contract(deployer, "address_whitelist").await?;
    let identifier_whitelist = declare_contract(deployer, "identifier_whitelist").await?;
    let store = declare_contract(deployer, "store").await?;
    let oracle_ancillary = declare_contract(deployer, "oracle_ancillary").await?;
    let optimistic_oracle_v1: FieldElement =
        declare_contract(deployer, "optimistic_oracle_v1").await?;
    Ok(Codes {
        finder,
        address_whitelist,
        identifier_whitelist,
        store,
        oracle_ancillary,
        optimistic_oracle_v1,
    })
}

pub async fn execute_call(
    contract: FieldElement,
    selector: &str,
    calldata: Vec<FieldElement>,
    owner: &StarknetAccount,
) -> FieldElement {
    let result = owner
        .execute(vec![Call {
            to: contract,
            selector: get_selector_from_name(selector).unwrap(),
            calldata: calldata,
        }])
        .send()
        .await
        .unwrap();
    result.transaction_hash
}
