use crate::types::{LibraryContracts, OODeploymentArguments, ASSERT_TRUTH};

use super::{
    types::{oracle_interfaces, Codes, OOConfig, StarknetAccount},
    utils::{deploy_contract, get_transaction_receipt},
};
use crate::bind::{
    address_whitelist::address_whitelist, finder::finder,
    identifier_whitelist::identifier_whitelist, store::store,
};
use anyhow::Result;
use cainome::cairo_serde::ContractAddress;
use log::{error, info};
use starknet::{accounts::Account, core::types::Felt, providers::Provider};
use starknet::{accounts::ConnectedAccount, macros::felt};

pub async fn deploy_core(
    owner: &StarknetAccount,
    deployer: &StarknetAccount,
    default_configuration: OOConfig,
    codes: &Codes,
) -> eyre::Result<Codes> {
    info!("Deploying the finder...");
    let finder = deploy_ownable_contract(codes.finder, deployer, owner).await?;

    info!("Finder deployed: {:x?}", finder);

    info!("Deploying address whitelist... ");
    let address_whitelist =
        deploy_ownable_contract(codes.address_whitelist, deployer, owner).await?;

    info!("Address whitelist deployed: {:x?}", address_whitelist);

    info!("Deploying identifier whitelist... ");
    let identifier_whitelist =
        deploy_ownable_contract(codes.identifier_whitelist, deployer, owner).await?;

    info!("Identifier whitelist deployed: {:x?}", identifier_whitelist);

    info!("Deploying Store... ");
    let store = deploy_ownable_contract(codes.store, deployer, owner).await?;

    info!("Store deployed: {:x?}", store);

    info!("Deploying Oracle ancillary... ");
    let oracle_ancillary =
        deploy_oracle_ancillary(codes.oracle_ancillary, deployer, owner, finder).await?;

    info!("Oracle ancillary deployed: {:x?}", oracle_ancillary);

    match configure_contracts(
        &default_configuration,
        LibraryContracts {
            finder,
            store,
            identifier_whitelist,
            address_whitelist,
            oracle: oracle_ancillary,
        },
        owner,
    )
    .await
    {
        Ok(_) => info!("Contract configuration completed successfully"),
        Err(e) => error!("An error occurred during contract configuration: {:?}", e),
    }

    info!("Deploying Optimistic oracle... ");
    let optimistic_oracle_v1 = deploy_optimistic_oracle(
        codes.optimistic_oracle_v1,
        deployer,
        owner,
        OODeploymentArguments {
            finder,
            erc20: default_configuration.erc20_token,
            liveness: default_configuration.liveness,
        },
    )
    .await?;

    info!(" Optimistic oracle deployed: {:x?}", optimistic_oracle_v1);

    Ok(Codes {
        finder,
        address_whitelist,
        identifier_whitelist,
        store,
        oracle_ancillary,
        optimistic_oracle_v1,
    })
}

pub async fn deploy_ownable_contract(
    class_hash: Felt,
    deployer: &StarknetAccount,
    owner: &StarknetAccount,
) -> eyre::Result<Felt> {
    let res = deploy_contract(class_hash, vec![owner.address()], deployer).await;
    Ok(res.0)
}

pub async fn deploy_oracle_ancillary(
    class_hash: Felt,
    deployer: &StarknetAccount,
    owner: &StarknetAccount,
    finder: Felt,
) -> eyre::Result<Felt> {
    let res = deploy_contract(class_hash, vec![finder], deployer).await;
    Ok(res.0)
}

pub async fn deploy_optimistic_oracle(
    class_hash: Felt,
    deployer: &StarknetAccount,
    owner: &StarknetAccount,
    oo_deployment_arguments: OODeploymentArguments,
) -> eyre::Result<Felt> {
    let res = deploy_contract(
        class_hash,
        vec![
            oo_deployment_arguments.finder,
            oo_deployment_arguments.erc20,
            oo_deployment_arguments.liveness.into(),
            owner.address(),
        ],
        deployer,
    )
    .await;
    Ok(res.0)
}

pub async fn configure_contracts(
    default_configuration: &OOConfig,
    contracts: LibraryContracts,
    owner: &StarknetAccount,
) -> Result<()> {
    let store = store::new(contracts.store, owner);
    let store_res = store
        .set_final_fee(
            &ContractAddress(default_configuration.erc20_token.into()),
            &default_configuration.final_fee,
        )
        .send()
        .await?;

    info!(
        "Set final fee for store contract with tx hash: {:x?}",
        store_res.transaction_hash
    );

    tokio::time::sleep(tokio::time::Duration::from_millis(10000)).await;
    assert!(
        get_transaction_receipt(owner.provider(), store_res.transaction_hash)
            .await
            .is_ok()
    );

    let identifier_whitelist = identifier_whitelist::new(contracts.identifier_whitelist, owner);
    let identifier_whitelist_res = identifier_whitelist
        .add_supported_identifier(&ASSERT_TRUTH)
        .send()
        .await?;

    info!(
        "Add supported identifer for identifier whitelist contract with tx hash: {:x?}",
        identifier_whitelist_res.transaction_hash
    );
    tokio::time::sleep(tokio::time::Duration::from_millis(10000)).await;

    assert!(
        get_transaction_receipt(owner.provider(), identifier_whitelist_res.transaction_hash)
            .await
            .is_ok()
    );

    let address_whitelist = address_whitelist::new(contracts.address_whitelist, owner);
    let address_whitelist_res = address_whitelist
        .add_to_whitelist(&ContractAddress(default_configuration.erc20_token))
        .send()
        .await?;

    info!(
        "Add collateral address whitelist for address whitelist contract with tx hash: {:x?}",
        address_whitelist_res.transaction_hash
    );

    assert!(
        get_transaction_receipt(owner.provider(), address_whitelist_res.transaction_hash)
            .await
            .is_ok()
    );

    let finder = finder::new(contracts.finder, owner);
    let finder_res = finder
        .change_implementation_address(
            &oracle_interfaces::OracleInterface::IDENTIFIER_WHITELIST.as_str(),
            &ContractAddress(contracts.identifier_whitelist),
        )
        .send()
        .await?;

    info!(
        "Set implementation address for IDENTIFIER_WHITELIST: {:x?}",
        finder_res.transaction_hash
    );
    tokio::time::sleep(tokio::time::Duration::from_millis(10000)).await;
    assert!(
        get_transaction_receipt(owner.provider(), finder_res.transaction_hash)
            .await
            .is_ok()
    );

    let finder_res = finder
        .change_implementation_address(
            &oracle_interfaces::OracleInterface::COLLATERAL_WHITELIST.as_str(),
            &ContractAddress(contracts.address_whitelist),
        )
        .send()
        .await?;

    info!(
        "Set implementation address for COLLATERAL_WHITELIST: {:x?}",
        finder_res.transaction_hash
    );

    tokio::time::sleep(tokio::time::Duration::from_millis(10000)).await;
    assert!(
        get_transaction_receipt(owner.provider(), finder_res.transaction_hash)
            .await
            .is_ok()
    );

    let finder_res = finder
        .change_implementation_address(
            &oracle_interfaces::OracleInterface::ORACLE.as_str(),
            &ContractAddress(contracts.oracle),
        )
        .send()
        .await?;

    info!(
        "Set implementation address for ORACLE: {:x?}",
        finder_res.transaction_hash
    );

    tokio::time::sleep(tokio::time::Duration::from_millis(10000)).await;
    assert!(
        get_transaction_receipt(owner.provider(), finder_res.transaction_hash)
            .await
            .is_ok()
    );

    let finder_res = finder
        .change_implementation_address(
            &oracle_interfaces::OracleInterface::STORE.as_str(),
            &ContractAddress(contracts.store),
        )
        .send()
        .await?;

    info!(
        "Set implementation address for STORE: {:x?}",
        finder_res.transaction_hash
    );

    tokio::time::sleep(tokio::time::Duration::from_millis(10000)).await;
    assert!(
        get_transaction_receipt(owner.provider(), finder_res.transaction_hash)
            .await
            .is_ok()
    );

    Ok(())
}
