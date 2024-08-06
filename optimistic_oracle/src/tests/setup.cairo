use snforge_std::{
    declare, ContractClassTrait, start_prank, stop_prank, CheatTarget, EventSpy, EventAssertions,
    spy_events, SpyOn
};
use optimistic_oracle::contracts::common::address_whitelist::address_whitelist::WhitelistType;
use starknet::{ContractAddress, contract_address_const, EthAddress};
use optimistic_oracle::contracts::interfaces::{
    IFinderDispatcher, IFinderDispatcherTrait, IOptimisticOracleDispatcher,
    IAddressWhitelistDispatcher, IAddressWhitelistDispatcherTrait, IIdentifierWhitelistDispatcher,
    IIdentifierWhitelistDispatcherTrait, IOracleAncillaryDispatcher,
    IMockOracleAncillaryConfigurationDispatcher, IStoreDispatcher, IStoreDispatcherTrait,
};
use openzeppelin::token::erc20::interface::{ERC20ABIDispatcher};
use optimistic_oracle::contracts::optimistic_oracle_v1::optimistic_oracle_v1::ORACLE_ADDRESS;
use openzeppelin::utils::serde::SerializedAppend;
use openzeppelin::access::ownable::interface::{IOwnableDispatcher, IOwnableDispatcherTrait};
use optimistic_oracle::contracts::utils::constants::OracleInterfaces;


pub const INITIAL_SUPPLY: u256 = 1000000000000000000000000000;
pub const DEFAULT_LIVENESS: u64 = 1000;
pub const FINAL_FEE: u256 = 10000;


pub fn OWNER() -> ContractAddress {
    contract_address_const::<'OWNER'>()
}

pub fn DISPUTER() -> ContractAddress {
    contract_address_const::<'DISPUTER'>()
}
fn NAME() -> ByteArray {
    "Currency"
}

fn SYMBOL() -> ByteArray {
    "CUR"
}

pub fn setup_mock_erc20() -> ERC20ABIDispatcher {
    let mock_erc20_class = declare("mock_erc20").unwrap();
    let (mock_erc20_addr, _) = mock_erc20_class
        .deploy(@array![INITIAL_SUPPLY.low.into(), INITIAL_SUPPLY.high.into(), OWNER().into()])
        .unwrap();
    ERC20ABIDispatcher { contract_address: mock_erc20_addr }
}

pub fn setup_store() -> IStoreDispatcher {
    let store_class = declare("store").unwrap();
    let (store_addr, _) = store_class.deploy(@array![OWNER().into()]).unwrap();
    IStoreDispatcher { contract_address: store_addr }
}

pub fn setup_finder() -> (IFinderDispatcher, EventSpy) {
    let finder_class = declare("finder").unwrap();
    let (finder_addr, _) = finder_class.deploy(@array![OWNER().into()]).unwrap();
    let mut spy = spy_events(SpyOn::One(finder_addr));
    (IFinderDispatcher { contract_address: finder_addr }, spy)
}

pub fn setup_address_whitelist() -> (IAddressWhitelistDispatcher, EventSpy) {
    let address_whitelist_class = declare("address_whitelist").unwrap();
    let (address_whitelist_addr, _) = address_whitelist_class
        .deploy(@array![OWNER().into()])
        .unwrap();
    let mut spy = spy_events(SpyOn::One(address_whitelist_addr));
    (IAddressWhitelistDispatcher { contract_address: address_whitelist_addr }, spy)
}

pub fn setup_identifier_whitelist() -> (IIdentifierWhitelistDispatcher, EventSpy) {
    let identifier_whitelist_class = declare("identifier_whitelist").unwrap();
    let (identifier_whitelist_addr, _) = identifier_whitelist_class
        .deploy(@array![OWNER().into()])
        .unwrap();
    let mut spy = spy_events(SpyOn::One(identifier_whitelist_addr));
    (IIdentifierWhitelistDispatcher { contract_address: identifier_whitelist_addr }, spy)
}

pub fn setup_optimistic_oracle(
    erc20: ERC20ABIDispatcher, finder: IFinderDispatcher, default_liveness: u64
) -> (IOptimisticOracleDispatcher, EventSpy) {
    let optimistic_oracle_class = declare("optimistic_oracle_v1").unwrap();
    setup_mock_oracle();
    let res = optimistic_oracle_class
        .deploy(
            @array![
                finder.contract_address.into(),
                erc20.contract_address.into(),
                default_liveness.into(),
                OWNER().into()
            ]
        );
    if (res.is_err()) {
        panic(res.unwrap_err())
    }
    let (optimistic_oracle_addr, _) = res.unwrap();
    let mut spy = spy_events(SpyOn::One(optimistic_oracle_addr));
    (IOptimisticOracleDispatcher { contract_address: optimistic_oracle_addr }, spy)
}

fn setup_mock_oracle() {
    let mock_oracle_class = declare("mock_oracle").unwrap();
    mock_oracle_class.deploy_at(@array![], ORACLE_ADDRESS.try_into().unwrap()).unwrap();
}


pub fn setup_mock_oracle_ancillary(
    finder: IFinderDispatcher
) -> (IOracleAncillaryDispatcher, IMockOracleAncillaryConfigurationDispatcher, EventSpy) {
    let oracle_ancillary_class = declare("mock_oracle_ancillary").unwrap();
    let (oracle_ancillary_addr, _) = oracle_ancillary_class
        .deploy(@array![finder.contract_address.into(), OWNER().into()])
        .unwrap();
    let mut spy = spy_events(SpyOn::One(oracle_ancillary_addr));
    (
        IOracleAncillaryDispatcher { contract_address: oracle_ancillary_addr },
        IMockOracleAncillaryConfigurationDispatcher { contract_address: oracle_ancillary_addr },
        spy
    )
}


pub fn oo_full_config() -> IOptimisticOracleDispatcher {
    let liveness = 30;
    let (finder, _) = setup_finder();
    let erc20 = setup_mock_erc20();
    let (oracle, _, _) = setup_mock_oracle_ancillary(finder);
    let store = setup_store();
    let ownable = IOwnableDispatcher { contract_address: store.contract_address };
    start_prank(CheatTarget::One(ownable.contract_address), OWNER());
    store.set_final_fee(erc20.contract_address, FINAL_FEE);
    let (address_whitelist, _) = setup_address_whitelist();
    let (identifier_whitelist, _) = setup_identifier_whitelist();
    let ownable = IOwnableDispatcher { contract_address: identifier_whitelist.contract_address };
    start_prank(CheatTarget::One(ownable.contract_address), OWNER());
    identifier_whitelist.add_supported_identifier(OracleInterfaces::IDENTIFIER_WHITELIST);
    let ownable = IOwnableDispatcher { contract_address: address_whitelist.contract_address };
    start_prank(CheatTarget::One(ownable.contract_address), OWNER());
    address_whitelist.add_to_whitelist(erc20.contract_address, WhitelistType::Currency);
    let ownable = IOwnableDispatcher { contract_address: finder.contract_address };
    start_prank(CheatTarget::One(ownable.contract_address), OWNER());
    finder
        .change_implementation_address(
            OracleInterfaces::IDENTIFIER_WHITELIST, identifier_whitelist.contract_address
        );
    finder.change_implementation_address(OracleInterfaces::ORACLE, oracle.contract_address);
    finder
        .change_implementation_address(
            OracleInterfaces::COLLATERAL_WHITELIST, address_whitelist.contract_address
        );
    finder.change_implementation_address(OracleInterfaces::STORE, store.contract_address);
    let (oo, _) = setup_optimistic_oracle(erc20, finder, liveness);
    oo
}
