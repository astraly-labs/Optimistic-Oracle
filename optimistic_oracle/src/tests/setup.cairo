use snforge_std::{
    declare, ContractClassTrait, CheatTarget, EventSpy, EventAssertions, spy_events, SpyOn
};
use starknet::{ContractAddress, contract_address_const, EthAddress};
use optimistic_oracle::contracts::interfaces::{
    IFinderDispatcher, IOptimisticOracleDispatcher, IAddressWhitelistDispatcher,
    IIdentifierWhitelistDispatcher
};
use openzeppelin::token::erc20::interface::{ERC20ABIDispatcher};
use openzeppelin::utils::serde::SerializedAppend;
use openzeppelin::access::ownable::interface::{IOwnableDispatcher, IOwnableDispatcherTrait};


pub const INITIAL_SUPPLY: u256 = 10000000000;
pub const DEFAULT_LIVENESS: u64 = 1000;

pub fn OWNER() -> ContractAddress {
    contract_address_const::<'OWNER'>()
}

fn NAME() -> ByteArray {
    "Currency"
}

fn SYMBOL() -> ByteArray {
    "CUR"
}

pub fn setup_mock_erc20() -> ERC20ABIDispatcher {
    let mut calldata: Array<felt252> = array![];
    calldata.append_serde(NAME());
    calldata.append_serde(SYMBOL());
    calldata.append(OWNER().into());
    let mock_erc20_class = declare("full_erc20").unwrap();
    let (mock_erc20_addr, _) = mock_erc20_class.deploy(@calldata).unwrap();
    ERC20ABIDispatcher { contract_address: mock_erc20_addr }
}

pub fn setup_finder() -> (IFinderDispatcher, EventSpy) {
    let finder_class = declare("finder").unwrap();
    let (finder_addr, _) = finder_class.deploy(@array![OWNER().into()]).unwrap();
    let mut spy = spy_events(SpyOn::One(finder_addr));
    (IFinderDispatcher { contract_address: finder_addr }, spy)
}
pub fn setup_oo() -> (IOptimisticOracleDispatcher, EventSpy) {
    let (finder, _) = setup_finder();
    let currency = setup_mock_erc20();
    let oo_class = declare("optimistic_oracle").unwrap();
    let (oo_addr, _) = oo_class
        .deploy(
            @array![
                finder.contract_address.into(),
                currency.contract_address.into(),
                DEFAULT_LIVENESS.into(),
                OWNER().into()
            ]
        )
        .unwrap();
    let mut spy = spy_events(SpyOn::One(oo_addr));
    (IOptimisticOracleDispatcher { contract_address: oo_addr }, spy)
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

pub fn setup_optimistic_oracle(default_liveness: u64) -> (IOptimisticOracleDispatcher, EventSpy) {
    let (finder, _) = setup_finder();
    let erc20 = setup_mock_erc20();
    let optimistic_oracle_class = declare("optimistic_oracle_v1").unwrap();
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

