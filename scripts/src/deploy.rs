use super::{
    types::{oracle_interfaces, Codes, OO_Config, StarknetAccount, ETH_ADDRESS},
    utils::{deploy_contract, execute_call},
};
use starknet::{
    accounts::{Account, Call},
    core::{types::FieldElement, utils::get_selector_from_name},
    macros::felt,
};

pub async fn deploy_core(
    owner: &StarknetAccount,
    deployer: &StarknetAccount,
    default_configuration: OO_Config,
    codes: &Codes,
) -> eyre::Result<Codes> {
    println!("Deploying the finder...");
    let finder = deploy_ownable_contract(codes.finder, deployer, owner).await?;

    println!("Finder deployed: {:x?}", finder);

    println!("Deploying address whitelist... ");
    let address_whitelist =
        deploy_ownable_contract(codes.address_whitelist, deployer, owner).await?;

    println!("Address whitelist deployed: {:x?}", address_whitelist);

    println!("Deploying identifier whitelist... ");
    let identifier_whitelist =
        deploy_ownable_contract(codes.identifier_whitelist, deployer, owner).await?;

    println!("Identifier whitelist deployed: {:x?}", identifier_whitelist);

    println!("Deploying Store... ");
    let store = deploy_ownable_contract(codes.store, deployer, owner).await?;

    println!("Store deployed: {:x?}", store);

    println!("Deploying Oracle ancillary... ");
    let oracle_ancillary =
        deploy_oracle_ancillary(codes.oracle_ancillary, deployer, owner, finder).await?;

    println!("Oracle ancillary deployed: {:x?}", oracle_ancillary);

    configure_contracts(
        &default_configuration,
        finder,
        store,
        identifier_whitelist,
        address_whitelist,
        oracle_ancillary,
        owner,
    )
    .await;

    println!("Deploying Optimistic oracle... ");
    let optimistic_oracle_v1 = deploy_optimistic_oracle(
        codes.optimistic_oracle_v1,
        deployer,
        owner,
        finder,
        default_configuration.erc20_token,
        default_configuration.liveness,
    )
    .await?;

    println!(" Optimistic oracle deployed: {:x?}", optimistic_oracle_v1);

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
    class_hash: FieldElement,
    deployer: &StarknetAccount,
    owner: &StarknetAccount,
) -> eyre::Result<FieldElement> {
    let res = deploy_contract(class_hash, vec![owner.address()], deployer).await;
    Ok(res.0)
}

pub async fn deploy_oracle_ancillary(
    class_hash: FieldElement,
    deployer: &StarknetAccount,
    owner: &StarknetAccount,
    finder: FieldElement,
) -> eyre::Result<FieldElement> {
    let res = deploy_contract(class_hash, vec![finder], deployer).await;
    Ok(res.0)
}

pub async fn deploy_optimistic_oracle(
    class_hash: FieldElement,
    deployer: &StarknetAccount,
    owner: &StarknetAccount,
    finder: FieldElement,
    erc20: FieldElement,
    liveness: u64,
) -> eyre::Result<FieldElement> {
    let res = deploy_contract(
        class_hash,
        vec![finder, erc20, liveness.into(), owner.address()],
        deployer,
    )
    .await;
    Ok(res.0)
}

pub async fn configure_contracts(
    default_configuration: &OO_Config,
    finder: FieldElement,
    store: FieldElement,
    identifier_whitelist: FieldElement,
    address_whitelist: FieldElement,
    oracle: FieldElement,
    owner: &StarknetAccount,
) {
    let store_calldata = vec![
        default_configuration.erc20_token,
        default_configuration.final_fee.low.into(),
        default_configuration.final_fee.high.into(),
    ]; // TODO : verify if there is no method more effective than this
    let store_tx_hash = execute_call(store, "set_final_fee", store_calldata, owner).await;

    println!(
        "Set final fee for store contract with tx hash: {:x?}",
        store_tx_hash
    );

    let identifier_whitelist_calldata = vec![felt!("0x4153534552545f5452555448")]; // "ASSERT_TRUTH"
    let identifier_whitelist_tx_hash = execute_call(
        identifier_whitelist,
        "add_supported_identifier",
        identifier_whitelist_calldata,
        owner,
    )
    .await;

    println!(
        "Add supported identifer for identifier whitelist contract with tx hash: {:x?}",
        identifier_whitelist_tx_hash
    );

    let address_whitelist_calldata = vec![default_configuration.erc20_token]; // "ASSERT_TRUTH"
    let address_whitelist_tx_hash = execute_call(
        address_whitelist,
        "add_to_whitelist",
        address_whitelist_calldata,
        owner,
    )
    .await;

    println!(
        "Add collateral address whitelist for address whitelist contract with tx hash: {:x?}",
        identifier_whitelist_tx_hash
    );

    let finder_iw_calldata = vec![
        oracle_interfaces::OracleInterface::IDENTIFIER_WHITELIST.as_str(),
        identifier_whitelist,
    ];
    let finder_iw_tx_hash = execute_call(
        finder,
        "change_implementation_address",
        finder_iw_calldata,
        owner,
    )
    .await;

    let finder_cw_calldata = vec![
        oracle_interfaces::OracleInterface::COLLATERAL_WHITELIST.as_str(),
        address_whitelist,
    ];
    let finder_cw_tx_hash = execute_call(
        finder,
        "change_implementation_address",
        finder_cw_calldata,
        owner,
    )
    .await;

    let finder_o_calldata = vec![oracle_interfaces::OracleInterface::ORACLE.as_str(), oracle];
    let finder_o_tx_hash = execute_call(
        finder,
        "change_implementation_address",
        finder_o_calldata,
        owner,
    )
    .await;

    let finder_s_calldata = vec![oracle_interfaces::OracleInterface::STORE.as_str(), store];
    let finder_s_tx_hash = execute_call(
        finder,
        "change_implementation_address",
        finder_s_calldata,
        owner,
    )
    .await;
}
