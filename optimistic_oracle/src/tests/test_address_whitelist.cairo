use starknet::{ContractAddress, contract_address_const};
use optimistic_oracle::tests::setup::{OWNER, setup_address_whitelist};
use openzeppelin::access::ownable::interface::{IOwnableDispatcher, IOwnableDispatcherTrait};
use snforge_std::{start_prank, CheatTarget, stop_prank};
use optimistic_oracle::contracts::{
    common::address_whitelist::address_whitelist,
    interfaces::{IAddressWhitelistDispatcher, IAddressWhitelistDispatcherTrait}
};
use snforge_std::cheatcodes::events::EventAssertions;
use optimistic_oracle::contracts::common::address_whitelist::address_whitelist::WhitelistType;


#[test]
fn test_address_whitelist_owner_verification() {
    let (address_whitelist, _) = setup_address_whitelist();
    let ownable = IOwnableDispatcher { contract_address: address_whitelist.contract_address };
    assert(ownable.owner() == OWNER(), 'AW: Wrong contract owner');
}

#[test]
fn test_address_whitelist_ugprade_ownership() {
    let (address_whitelist, _) = setup_address_whitelist();
    let new_owner: ContractAddress = 'NEW_OWNER'.try_into().unwrap();
    let ownable = IOwnableDispatcher { contract_address: address_whitelist.contract_address };
    start_prank(CheatTarget::One(ownable.contract_address), OWNER());
    ownable.transfer_ownership(new_owner);
    assert(ownable.owner() == new_owner, 'AW: Transfer ownership failed');
}

#[test]
fn test_address_whitelist_add_to_whitelist() {
    // 1st insertion 
    let address_to_add = contract_address_const::<0x234e2>();
    let (address_whitelist, mut spy) = setup_address_whitelist();
    let ownable = IOwnableDispatcher { contract_address: address_whitelist.contract_address };
    start_prank(CheatTarget::One(ownable.contract_address), OWNER());
    address_whitelist.add_to_whitelist(address_to_add, WhitelistType::Currency);
    let expected_event = address_whitelist::Event::AddedToWhitelist(
        address_whitelist::AddedToWhitelist {
            added_address: address_to_add, whitelist_type: WhitelistType::Currency
        }
    );
    spy.assert_emitted(@array![(address_whitelist.contract_address, expected_event)]);
    let mut expected_result = array![address_to_add];
    assert(
        address_whitelist.get_whitelist(WhitelistType::Currency) == expected_result.span(),
        'AW: Insertion failed'
    );
    assert_eq!(address_whitelist.is_on_whitelist(address_to_add, WhitelistType::Currency), true);

    // 2nd insertion
    let new_address_to_add = contract_address_const::<0x1231413>();
    address_whitelist.add_to_whitelist(new_address_to_add, WhitelistType::Currency);
    expected_result.append(new_address_to_add);
    let expected_event = address_whitelist::Event::AddedToWhitelist(
        address_whitelist::AddedToWhitelist {
            added_address: new_address_to_add, whitelist_type: WhitelistType::Currency
        }
    );
    spy.assert_emitted(@array![(address_whitelist.contract_address, expected_event)]);
    assert(
        address_whitelist.get_whitelist(WhitelistType::Currency) == expected_result.span(),
        'AW: 2 Insertion failed'
    );
    assert_eq!(
        address_whitelist.is_on_whitelist(new_address_to_add, WhitelistType::Currency), true
    );
    // Duplicate insertion
    address_whitelist.add_to_whitelist(address_to_add, WhitelistType::Currency);
    assert(
        address_whitelist.get_whitelist(WhitelistType::Currency) == expected_result.span(),
        'AW: Duplicate insertion failed'
    );
    assert_eq!(address_whitelist.is_on_whitelist(address_to_add, WhitelistType::Currency), true);
}


#[test]
fn test_address_whitelist_remove_from_whitelist() {
    // 1st insertion 
    let address_to_add = contract_address_const::<0x234e2>();
    let (address_whitelist, mut spy) = setup_address_whitelist();
    let ownable = IOwnableDispatcher { contract_address: address_whitelist.contract_address };
    start_prank(CheatTarget::One(ownable.contract_address), OWNER());
    address_whitelist.add_to_whitelist(address_to_add, WhitelistType::Currency);
    let expected_event = address_whitelist::Event::AddedToWhitelist(
        address_whitelist::AddedToWhitelist {
            added_address: address_to_add, whitelist_type: WhitelistType::Currency
        }
    );
    spy.assert_emitted(@array![(address_whitelist.contract_address, expected_event)]);
    let mut expected_result = array![address_to_add];
    assert(
        address_whitelist.get_whitelist(WhitelistType::Currency) == expected_result.span(),
        'AW: Insertion failed'
    );

    // Removal 
    address_whitelist.remove_from_whitelist(address_to_add, WhitelistType::Currency);
    assert(
        address_whitelist.get_whitelist(WhitelistType::Currency) == array![].span(),
        'AW: Removal failed'
    );
    assert_eq!(address_whitelist.is_on_whitelist(address_to_add, WhitelistType::Currency), false);
    let expected_event = address_whitelist::Event::RemovedFromWhitelist(
        address_whitelist::RemovedFromWhitelist {
            removed_address: address_to_add, whitelist_type: WhitelistType::Currency
        }
    );
    spy.assert_emitted(@array![(address_whitelist.contract_address, expected_event)]);
    // Reintroduction

    address_whitelist.add_to_whitelist(address_to_add, WhitelistType::Currency);
    assert(
        address_whitelist.get_whitelist(WhitelistType::Currency) == expected_result.span(),
        'AW: Reinsertion failed'
    );
    assert_eq!(address_whitelist.is_on_whitelist(address_to_add, WhitelistType::Currency), true);
}

#[test]
#[should_panic(expected: ('Caller is not the owner',))]
fn test_address_whitelist_add_to_whitelist_fails_if_caller_not_owner() {
    let address_to_add = contract_address_const::<0x234e2>();
    let (address_whitelist, _) = setup_address_whitelist();
    address_whitelist.add_to_whitelist(address_to_add, WhitelistType::Currency);
}

