use starknet::{ContractAddress, contract_address_const};
use openzeppelin::access::ownable::interface::{IOwnableDispatcher, IOwnableDispatcherTrait};
use snforge_std::{start_prank, CheatTarget, stop_prank, start_warp, stop_warp};
use optimistic_oracle::tests::setup::{
    OWNER, DISPUTER, INITIAL_SUPPLY, oo_full_config, setup_optimistic_oracle,
    setup_identifier_whitelist, setup_finder, setup_address_whitelist, setup_store,
    setup_mock_erc20, FINAL_FEE, setup_mock_oracle_ancillary
};
use optimistic_oracle::contracts::{
    interfaces::{
        IOptimisticOracleDispatcher, IOptimisticOracleDispatcherTrait, IStoreDispatcherTrait,
        IFinderDispatcherTrait, IIdentifierWhitelistDispatcherTrait,
        IAddressWhitelistDispatcherTrait, IMockOracleAncillaryConfigurationDispatcherTrait
    }, 
    optimistic_oracle_v1::optimistic_oracle_v1, 
};
use optimistic_oracle::contracts::common::address_whitelist::address_whitelist::WhitelistType;
use snforge_std::cheatcodes::events::EventAssertions;
use optimistic_oracle::contracts::utils::constants::OracleInterfaces;
use openzeppelin::token::erc20::interface::{ERC20ABI, ERC20ABIDispatcher, ERC20ABIDispatcherTrait};

const DEFAULT_PRICE: u256= 400000000000000;
#[test]
fn test_oo_owner_verification() {
    let oo = oo_full_config();
    let ownable = IOwnableDispatcher { contract_address: oo.contract_address };
    assert(ownable.owner() == OWNER(), 'Wrong contract owner');
}

#[test]
fn test_oo_upgrade_ownership() {
    let oo = oo_full_config();
    let new_owner: ContractAddress = 'NEW_OWNER'.try_into().unwrap();
    let ownable = IOwnableDispatcher { contract_address: oo.contract_address };
    start_prank(CheatTarget::One(ownable.contract_address), OWNER());
    ownable.transfer_ownership(new_owner);
    assert(ownable.owner() == new_owner, 'Transfer ownership failed');
}

#[test]
fn test_oo_get_default_identifier() {
    let oo = oo_full_config();
    assert_eq!(oo.default_identifier(), optimistic_oracle_v1::DEFAULT_IDENTIFIER);
}

#[test]
fn test_oo_assert_truth_with_default() {
    // Manual setup definition 
    let liveness = 30;
    let (finder, _) = setup_finder();
    let erc20 = setup_mock_erc20();
    let (oracle, oracle_config, _) = setup_mock_oracle_ancillary(finder);
    let store = setup_store();
    let ownable = IOwnableDispatcher { contract_address: store.contract_address };
    start_prank(CheatTarget::One(ownable.contract_address), OWNER());
    store.set_final_fee(erc20.contract_address, FINAL_FEE);
    let (address_whitelist, _) = setup_address_whitelist();
    let (identifier_whitelist, _) = setup_identifier_whitelist();
    let ownable = IOwnableDispatcher { contract_address: identifier_whitelist.contract_address };
    start_prank(CheatTarget::One(ownable.contract_address), OWNER());
    identifier_whitelist.add_supported_identifier(optimistic_oracle_v1::DEFAULT_IDENTIFIER);
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

    let mut claim: ByteArray = Default::default();
    // Claim: Argentina won the world cup 2022
    claim.append_word(0x41, 1);
    claim.append_word(0x7267656e74696e6120776f6e2074686520776f726c64206375702032303232, 31);

    // define the allowance for the contract
    let minimum_bond = oo.get_minimum_bond(erc20.contract_address);
    let ownable_erc20 = IOwnableDispatcher { contract_address: erc20.contract_address };
    start_prank(CheatTarget::One(ownable_erc20.contract_address), OWNER());
    erc20.approve(oo.contract_address, minimum_bond + DEFAULT_PRICE.into());
    stop_prank(CheatTarget::One(ownable_erc20.contract_address));
    let ownable = IOwnableDispatcher { contract_address: oo.contract_address };
    start_prank(CheatTarget::One(ownable.contract_address), OWNER());
    let time = starknet::get_block_timestamp();
    let assertion_id = oo.assert_truth_with_defaults(claim.clone(), OWNER());
    let assertion = oo.get_assertion(assertion_id);
    assert_eq!(assertion.asserter, OWNER());
    assert_eq!(assertion.disputer, contract_address_const::<0>());
    assert_eq!(assertion.callback_recipient, contract_address_const::<0>());
    assert_eq!(assertion.currency.contract_address, erc20.contract_address);
    assert_eq!(assertion.domain_id, 0);
    assert_eq!(assertion.identifier, 'ASSERT_TRUTH');
    assert_eq!(assertion.bond, minimum_bond);
    assert_eq!(assertion.settled, false);
    assert_eq!(assertion.settlement_resolution, false);
    assert_eq!(assertion.assertion_time, time);
    assert_eq!(assertion.expiration_time, time + liveness);
    assert_eq!(erc20.balanceOf(OWNER()), INITIAL_SUPPLY - minimum_bond - DEFAULT_PRICE.into());

    // now we can settle the assertion, without dispute
    start_warp(CheatTarget::One(oo.contract_address), starknet::get_block_timestamp() + liveness);
    oo.settle_assertion(assertion_id);
    let assertion = oo.get_assertion(assertion_id);
    assert_eq!(assertion.asserter, OWNER());
    assert_eq!(assertion.disputer, contract_address_const::<0>());
    assert_eq!(assertion.callback_recipient, contract_address_const::<0>());
    assert_eq!(assertion.currency.contract_address, erc20.contract_address);
    assert_eq!(assertion.domain_id, 0);
    assert_eq!(assertion.identifier, 'ASSERT_TRUTH');
    assert_eq!(assertion.bond, minimum_bond);
    assert_eq!(assertion.settled, true);
    assert_eq!(assertion.settlement_resolution, true);
    assert_eq!(assertion.assertion_time, time);
    assert_eq!(assertion.expiration_time, time + liveness);
    assert_eq!(erc20.balanceOf(OWNER()), INITIAL_SUPPLY - DEFAULT_PRICE.into());
    stop_warp(CheatTarget::One(oo.contract_address));


    // now we initiate a new assertion with a dispute process, but first, we need to provide the disputer with the necessary funds
    start_prank(CheatTarget::One(ownable_erc20.contract_address), OWNER());
    erc20.approve(oo.contract_address, minimum_bond + DEFAULT_PRICE.into());
    stop_prank(CheatTarget::One(ownable_erc20.contract_address));
    claim.append_word(0x1234, 2);
    let assertion_id = oo.assert_truth_with_defaults(claim, OWNER());
    start_prank(CheatTarget::One(ownable_erc20.contract_address), OWNER());
    erc20.transfer(DISPUTER(), minimum_bond);
    stop_prank(CheatTarget::One(ownable_erc20.contract_address));
    start_prank(CheatTarget::One(ownable_erc20.contract_address), DISPUTER());
    erc20.approve(oo.contract_address, erc20.balanceOf(DISPUTER()));
    stop_prank(CheatTarget::One(ownable_erc20.contract_address));
    start_prank(CheatTarget::One(ownable.contract_address), DISPUTER());
    start_warp(
        CheatTarget::One(oo.contract_address), starknet::get_block_timestamp() + liveness - 1
    );

    oo.dispute_assertion(assertion_id, DISPUTER());

    let assertion = oo.get_assertion(assertion_id);
    assert_eq!(assertion.asserter, OWNER());
    assert_eq!(assertion.disputer, DISPUTER());
    assert_eq!(assertion.callback_recipient, contract_address_const::<0>());
    assert_eq!(assertion.currency.contract_address, erc20.contract_address);
    assert_eq!(assertion.domain_id, 0);
    assert_eq!(assertion.identifier, 'ASSERT_TRUTH');
    assert_eq!(assertion.bond, minimum_bond);
    assert_eq!(assertion.settled, false);
    assert_eq!(assertion.settlement_resolution, false);
    assert_eq!(assertion.assertion_time, time);
    assert_eq!(assertion.expiration_time, time + liveness);
    assert_eq!(erc20.balanceOf(DISPUTER()), 0);
    stop_warp(CheatTarget::One(oo.contract_address));

    // we need to settle the price on the ancillary oracle
    oracle_config
        .push_price(
            'ASSERT_TRUTH', time.into(), oo.stamp_assertion(assertion_id), 1000000000000000000
        );

    // once dispute is done, we can settle and conclude the process

    oo.settle_assertion(assertion_id);

    let assertion = oo.get_assertion(assertion_id);
    assert_eq!(assertion.asserter, OWNER());
    assert_eq!(assertion.disputer, DISPUTER());
    assert_eq!(assertion.callback_recipient, contract_address_const::<0>());
    assert_eq!(assertion.currency.contract_address, erc20.contract_address);
    assert_eq!(assertion.domain_id, 0);
    assert_eq!(assertion.identifier, 'ASSERT_TRUTH');
    assert_eq!(assertion.bond, minimum_bond);
    assert_eq!(assertion.settled, true);
    assert_eq!(assertion.settlement_resolution, true);
    assert_eq!(assertion.assertion_time, time);
    assert_eq!(assertion.expiration_time, time + liveness);
    assert_eq!(erc20.balanceOf(DISPUTER()), 0);
    // So the owner balance should be the initial amount -  the amount sent to the oracle (because the owner sent minimum_bond to the disputer before the dispute process)
    assert_eq!(erc20.balanceOf(OWNER()), INITIAL_SUPPLY - minimum_bond / 2 - 2*DEFAULT_PRICE.into() );
    assert_eq!(erc20.balanceOf(store.contract_address), minimum_bond / 2);
    stop_warp(CheatTarget::One(oo.contract_address));
}
