use crate::types::{LibraryContracts, OODeploymentArguments, ASSERT_TRUTH};

use super::{
    types::{oracle_interfaces, Codes, OOConfig, StarknetAccount},
    utils::{deploy_contract, execute_call},
};
use log::info;
use starknet::{accounts::Account, core::types::Felt};
use crate::bind::{finder::finder};

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

    configure_contracts(
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
    .await;

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
) {
    let store_calldata = vec![
        default_configuration.erc20_token,
        default_configuration.final_fee.low.into(),
        default_configuration.final_fee.high.into(),
    ]; // TODO : verify if there is no method more effective than this
    let store_tx_hash = execute_call(contracts.store, "set_final_fee", store_calldata, owner).await;

    info!(
        "Set final fee for store contract with tx hash: {:x?}",
        store_tx_hash
    );

    let identifier_whitelist_calldata = vec![ASSERT_TRUTH];
    let identifier_whitelist_tx_hash = execute_call(
        contracts.identifier_whitelist,
        "add_supported_identifier",
        identifier_whitelist_calldata,
        owner,
    )
    .await;

    info!(
        "Add supported identifer for identifier whitelist contract with tx hash: {:x?}",
        identifier_whitelist_tx_hash
    );

    let address_whitelist_calldata = vec![default_configuration.erc20_token]; // "ASSERT_TRUTH"
    let address_whitelist_tx_hash = execute_call(
        contracts.address_whitelist,
        "add_to_whitelist",
        address_whitelist_calldata,
        owner,
    )
    .await;

    info!(
        "Add collateral address whitelist for address whitelist contract with tx hash: {:x?}",
        identifier_whitelist_tx_hash
    );

    let finder_iw_calldata = vec![
        oracle_interfaces::OracleInterface::IDENTIFIER_WHITELIST.as_str(),
        contracts.identifier_whitelist,
    ];
    let finder_iw_tx_hash = execute_call(
        contracts.finder,
        "change_implementation_address",
        finder_iw_calldata,
        owner,
    )
    .await;

    let finder_cw_calldata = vec![
        oracle_interfaces::OracleInterface::COLLATERAL_WHITELIST.as_str(),
        contracts.address_whitelist,
    ];
    let finder_cw_tx_hash = execute_call(
        contracts.finder,
        "change_implementation_address",
        finder_cw_calldata,
        owner,
    )
    .await;

    let finder_o_calldata = vec![
        oracle_interfaces::OracleInterface::ORACLE.as_str(),
        contracts.oracle,
    ];
    let finder_o_tx_hash = execute_call(
        contracts.finder,
        "change_implementation_address",
        finder_o_calldata,
        owner,
    )
    .await;

    let finder_s_calldata = vec![
        oracle_interfaces::OracleInterface::STORE.as_str(),
        contracts.store,
    ];
    let finder_s_tx_hash = execute_call(
        contracts.finder,
        "change_implementation_address",
        finder_s_calldata,
        owner,
    )
    .await;
}
