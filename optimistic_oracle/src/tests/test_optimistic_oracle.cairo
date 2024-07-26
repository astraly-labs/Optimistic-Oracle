use starknet::{ContractAddress, contract_address_const};
use openzeppelin::access::ownable::interface::{IOwnableDispatcher, IOwnableDispatcherTrait};
use snforge_std::{start_prank, CheatTarget, stop_prank};
use optimistic_oracle::tests::setup::{OWNER, setup_optimistic_oracle};
use optimistic_oracle::contracts::{
    interfaces::{IOptimisticOracleDispatcher, IOptimisticOracleDispatcherTrait}
};
use optimistic_oracle::contracts::optimistic_oracle_v1::optimistic_oracle_v1;
use snforge_std::cheatcodes::events::EventAssertions;

#[test]
fn test_oo_owner_verification() {
    let liveness = 30;
    let (oo, _) = setup_optimistic_oracle(liveness);
    let ownable = IOwnableDispatcher { contract_address: oo.contract_address };
    assert(ownable.owner() == OWNER(), 'Wrong contract owner');
}

#[test]
fn test_oo_upgrade_ownership() {
    let liveness = 30;
    let (oo, _) = setup_optimistic_oracle(liveness);
    let new_owner: ContractAddress = 'NEW_OWNER'.try_into().unwrap();
    let ownable = IOwnableDispatcher { contract_address: oo.contract_address };
    start_prank(CheatTarget::One(ownable.contract_address), OWNER());
    ownable.transfer_ownership(new_owner);
    assert(ownable.owner() == new_owner, 'Transfer ownership failed');
}

#[test]
fn test_oo_get_default_identifier() {
    let liveness = 30;
    let (oo, _) = setup_optimistic_oracle(liveness);
    assert_eq!(oo.default_identifier(), optimistic_oracle_v1::DEFAULT_IDENTIFIER);
}

#[test]
fn test_oo_assert_truth_with_default() {
    let liveness = 30;
    let (oo, _) = setup_optimistic_oracle(liveness);
    assert_eq!(oo.default_identifier(), optimistic_oracle_v1::DEFAULT_IDENTIFIER);
}

