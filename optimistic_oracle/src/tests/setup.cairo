use snforge_std::{
    declare, ContractClassTrait, CheatTarget, EventSpy, EventAssertions, spy_events, SpyOn
};
use starknet::{ContractAddress, contract_address_const, EthAddress};
use optimistic_oracle::contracts::interfaces::{IFinderDispatcher, IOptimisticOracleDispatcher, IAddressWhitelistDispatcher};
use openzeppelin::token::erc20::interface::{ERC20ABIDispatcher};


pub const INITIAL_SUPPLY: u256 = 10000000000;
pub const DEFAULT_LIVENESS: u64 = 1000; 

pub fn OWNER() -> ContractAddress {
    contract_address_const::<'OWNER'>()
}

pub fn setup_mock_erc20() -> ERC20ABIDispatcher{
    let mock_erc20_class = declare("mock_erc20").unwrap();
    let (mock_erc20_addr, _) = mock_erc20_class.deploy( @array![INITIAL_SUPPLY.low.into(), INITIAL_SUPPLY.high.into(), OWNER().into()]).unwrap(); 
    ERC20ABIDispatcher{contract_address: mock_erc20_addr}
}

pub fn setup_finder() -> (IFinderDispatcher, EventSpy) {
    let finder_class = declare("finder").unwrap();
    let (finder_addr, _) = finder_class.deploy(@array![OWNER().into()]).unwrap();
    let mut spy = spy_events(SpyOn::One(finder_addr));
    (IFinderDispatcher{contract_address: finder_addr},spy)

}
pub fn setup_oo() -> (IOptimisticOracleDispatcher, EventSpy){
    let (finder, _) = setup_finder();
    let currency = setup_mock_erc20();
    let oo_class = declare("optimistic_oracle").unwrap();
    let (oo_addr, _) = oo_class.deploy(@array![finder.contract_address.into(),currency.contract_address.into(),DEFAULT_LIVENESS.into(), OWNER().into()]).unwrap();
    let mut spy = spy_events(SpyOn::One(oo_addr));
    (IOptimisticOracleDispatcher{contract_address: oo_addr}, spy)
}

pub fn setup_address_whitelist() -> (IAddressWhitelistDispatcher, EventSpy) {
    let address_whitelist_class = declare("address_whitelist").unwrap();
    let  (address_whitelist_addr, _) = address_whitelist_class.deploy(@array![OWNER().into()]).unwrap();
    let mut spy = spy_events(SpyOn::One(address_whitelist_addr));
    (IAddressWhitelistDispatcher{contract_address: address_whitelist_addr}, spy)
}