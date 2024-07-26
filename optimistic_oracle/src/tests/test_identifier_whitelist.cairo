use starknet::{ContractAddress, contract_address_const};
use optimistic_oracle::tests::setup::{OWNER, setup_identifier_whitelist};
use openzeppelin::access::ownable::interface::{IOwnableDispatcher, IOwnableDispatcherTrait};
use optimistic_oracle::contracts::{
    data_verification::identifier_whitelist::identifier_whitelist,
    interfaces::{IIdentifierWhitelistDispatcher, IIdentifierWhitelistDispatcherTrait}
};
use snforge_std::{start_prank, CheatTarget, stop_prank};
use snforge_std::cheatcodes::events::EventAssertions;


#[test]
fn test_identifier_whitelist_owner_verification() {
    let (identifier_whitelist, _) = setup_identifier_whitelist();
    let ownable = IOwnableDispatcher { contract_address: identifier_whitelist.contract_address };
    assert(ownable.owner() == OWNER(), 'IW: Wrong contract owner');
}

#[test]
fn test_identifier_whitelist_upgrade_ownership() {
    let (identifier_whitelist, _) = setup_identifier_whitelist();
    let new_owner: ContractAddress = 'NEW_OWNER'.try_into().unwrap();
    let ownable = IOwnableDispatcher { contract_address: identifier_whitelist.contract_address };
    start_prank(CheatTarget::One(ownable.contract_address), OWNER());
    ownable.transfer_ownership(new_owner);
    assert(ownable.owner() == new_owner, 'IW: Transfer ownership failed');
}

#[test]
fn test_identifier_whitelist_add_supported_identifier() {
    let new_identifier = 'NEW_IDENTIFIER';
    let (identifier_whitelist, mut spy) = setup_identifier_whitelist();
    let ownable = IOwnableDispatcher { contract_address: identifier_whitelist.contract_address };
    start_prank(CheatTarget::One(ownable.contract_address), OWNER());
    identifier_whitelist.add_supported_identifier(new_identifier);
    let expected_event = identifier_whitelist::Event::SupportedIdentifierAdded(
        identifier_whitelist::SupportedIdentifierAdded { identifier: new_identifier }
    );
    spy.assert_emitted(@array![(identifier_whitelist.contract_address, expected_event)]);
    assert(
        identifier_whitelist.is_identifier_supported(new_identifier), 'IW: add supported failed'
    );
}

#[test]
#[should_panic(expected: ('Caller is not the owner',))]
fn test_identifier_whitelist_add_support_identifier_fails_if_caller_not_owner() {
    let new_identifier = 'NEW_IDENTIFIER';
    let (identifier_whitelist, _) = setup_identifier_whitelist();
    identifier_whitelist.add_supported_identifier(new_identifier);
}


#[test]
fn test_identifier_whitelist_remove_supported_identifier() {
    let identifier = 'IDENTIFIER';
    let (identifier_whitelist, mut spy) = setup_identifier_whitelist();
    let ownable = IOwnableDispatcher { contract_address: identifier_whitelist.contract_address };
    start_prank(CheatTarget::One(ownable.contract_address), OWNER());
    identifier_whitelist.add_supported_identifier(identifier);
    let expected_event = identifier_whitelist::Event::SupportedIdentifierAdded(
        identifier_whitelist::SupportedIdentifierAdded { identifier: identifier }
    );
    spy.assert_emitted(@array![(identifier_whitelist.contract_address, expected_event)]);
    assert(identifier_whitelist.is_identifier_supported(identifier), 'IW: add supported failed');
    identifier_whitelist.remove_supported_identifier(identifier);
    let expected_event = identifier_whitelist::Event::SupportedIdentifierRemoved(
        identifier_whitelist::SupportedIdentifierRemoved { identifier: identifier }
    );
    spy.assert_emitted(@array![(identifier_whitelist.contract_address, expected_event)]);
    assert(
        !identifier_whitelist.is_identifier_supported(identifier), 'IW: remove supported failed'
    );
}

