use starknet::{ContractAddress, contract_address_const}; 
use openzeppelin::access::ownable::interface::{IOwnableDispatcher, IOwnableDispatcherTrait};
use snforge_std::{start_prank, CheatTarget, stop_prank};
use optimistic_oracle::tests::setup::{OWNER, setup_finder};
use optimistic_oracle::contracts::{finder::finder,interfaces::{IFinderDispatcher, IFinderDispatcherTrait}};
use snforge_std::cheatcodes::events::EventAssertions;

#[test]
fn test_finder_owner_verification(){
    let (finder, _) = setup_finder();
    let ownable = IOwnableDispatcher { contract_address: finder.contract_address };
    assert(ownable.owner() == OWNER(), 'Wrong contract owner');
}

#[test]
fn test_finder_ugprade_ownership(){
    let (finder, _) = setup_finder();
    let new_owner: ContractAddress = 'NEW_OWNER'.try_into().unwrap();    
    let ownable = IOwnableDispatcher{contract_address: finder.contract_address};
    start_prank(CheatTarget::One(ownable.contract_address), OWNER());
    ownable.transfer_ownership(new_owner);
    assert(ownable.owner() == new_owner,'Transfer ownership failed');
}


#[test]
fn test_finder_change_implementation_address(){
    let (finder, mut spy) = setup_finder();
    let interface_name ='interface_name'; 
    let new_implementation_address = contract_address_const::<0x12345>();
    let ownable = IOwnableDispatcher { contract_address: finder.contract_address };
    start_prank(CheatTarget::One(ownable.contract_address), OWNER());
    finder.change_implementation_address(interface_name, new_implementation_address);
    assert_eq!(new_implementation_address, finder.get_implementation_address(interface_name));
    let expected_event = finder::Event::InterfaceImplementationChanged(
        finder::InterfaceImplementationChanged { interface_name: interface_name, new_implementation_address:new_implementation_address  }
    );
    spy.assert_emitted(@array![(finder.contract_address, expected_event)]);
}

#[test]
#[should_panic(expected: ('Implementation not found',))]
fn tes_finder_get_implementation_address_fails_if_no_implementation_defined(){
    let (finder, _) = setup_finder(); 
    finder.get_implementation_address('not_registered');
}


#[test]
#[should_panic(expected: ('Caller is not the owner',))]
fn test_finder_change_implementation_address_fails_if_not_owner(){
    let (finder, _) = setup_finder(); 
    finder.change_implementation_address('new_interface',contract_address_const::<0x1234>());
}

