use optimistic_oracle::contracts::interfaces::IFinderDispatcherTrait;
use snforge_std::{
    declare, ContractClassTrait, spy_events, CheatTarget, start_prank, SpyOn, EventSpy,
    EventAssertions
};
use starknet::{ContractAddress, contract_address_const};
use optimistic_oracle::contracts::interfaces::{
    IOracleAncillary, IOracleAncillaryDispatcher, IOracleAncillaryDispatcherTrait,
    IMockOracleAncillaryConfiguration, IMockOracleAncillaryConfigurationDispatcherTrait,
    IMockOracleAncillaryConfigurationDispatcher, IIdentifierWhitelistDispatcher,
    IIdentifierWhitelistDispatcherTrait
};
use optimistic_oracle::contracts::mocks::oracle_ancillary::mock_oracle_ancillary;
use optimistic_oracle::contracts::utils::convert::convert_byte_array_to_felt_array;
use optimistic_oracle::tests::setup::{
    OWNER, setup_mock_oracle_ancillary, setup_finder, setup_identifier_whitelist
};
use core::poseidon::poseidon_hash_span;
use openzeppelin::access::ownable::interface::{IOwnableDispatcher, IOwnableDispatcherTrait};
use alexandria_data_structures::array_ext::ArrayTraitExt;
use optimistic_oracle::contracts::utils::constants::OracleInterfaces;


// Helper function to encode price request (copied from the contract)
fn encode_price_request(identifier: felt252, time: u256, ancillary_data: @ByteArray) -> felt252 {
    let input: Array<felt252> = array![identifier, time.high.into(), time.low.into()];
    let mut ancillary_data_felt: Array<felt252> = convert_byte_array_to_felt_array(ancillary_data);
    input.concat(@ancillary_data_felt);
    poseidon_hash_span(input.span())
}

#[test]
fn test_request_price() {
   let (finder, _) = setup_finder(); 
    let (oracle, _, mut spy) = setup_mock_oracle_ancillary(finder);
    let (identifier_whitelist, _) = setup_identifier_whitelist();
    let ownable = IOwnableDispatcher { contract_address: identifier_whitelist.contract_address };
    start_prank(CheatTarget::One(ownable.contract_address), OWNER());
    identifier_whitelist.add_supported_identifier(OracleInterfaces::IDENTIFIER_WHITELIST);
    let ownable = IOwnableDispatcher { contract_address: finder.contract_address };
    start_prank(CheatTarget::One(ownable.contract_address), OWNER());
    finder.change_implementation_address(OracleInterfaces::IDENTIFIER_WHITELIST, identifier_whitelist.contract_address); 

    let ownable = IOwnableDispatcher { contract_address: oracle.contract_address };
    start_prank(CheatTarget::One(ownable.contract_address), OWNER());
    let identifier = OracleInterfaces::IDENTIFIER_WHITELIST; 
    let time: u256 = 1000;
    let ancillary_data: ByteArray = Default::default(); 

    oracle.request_price(identifier, time, ancillary_data.clone());

    let expected_event =  mock_oracle_ancillary::Event::PriceRequestAdded(mock_oracle_ancillary::PriceRequestAdded {
        requester: OWNER(),
        identifier: identifier,
        time: time,
        ancillary_data: ancillary_data.clone(),
        request_id: encode_price_request(identifier, time, @ancillary_data),
    });
    spy.assert_emitted(@array![(oracle.contract_address, expected_event)]);

    oracle.request_price(identifier, time, ancillary_data);
    assert_eq!(spy.events.len(), 0, "Should not emit event for duplicate request");
}

#[test]
#[should_panic(expected: ('Identifier not supported',))]
fn test_request_price_unsupported_identifier() {
   let (finder, _) = setup_finder(); 
    let (oracle, _, _) = setup_mock_oracle_ancillary(finder);
    let (identifier_whitelist, _) = setup_identifier_whitelist();
    let ownable = IOwnableDispatcher { contract_address: finder.contract_address };
    let unsupported_identifier: felt252 = 5678;
    start_prank(CheatTarget::One(ownable.contract_address), OWNER());
    finder.change_implementation_address(OracleInterfaces::IDENTIFIER_WHITELIST, identifier_whitelist.contract_address); 
    let time: u256 = 1000;
    let ancillary_data: ByteArray = Default::default(); 

    oracle.request_price(unsupported_identifier, time, ancillary_data);
}

#[test]
fn test_push_price() {
    let (finder, _) = setup_finder();
    let (oracle, oracle_config, mut spy) = setup_mock_oracle_ancillary(finder);
    let (identifier_whitelist, _) = setup_identifier_whitelist();
    let ownable = IOwnableDispatcher { contract_address: identifier_whitelist.contract_address };
    start_prank(CheatTarget::One(ownable.contract_address), OWNER());
    identifier_whitelist.add_supported_identifier(OracleInterfaces::IDENTIFIER_WHITELIST);
    let ownable = IOwnableDispatcher { contract_address: finder.contract_address };
    start_prank(CheatTarget::One(ownable.contract_address), OWNER());
    finder
        .change_implementation_address(
            OracleInterfaces::IDENTIFIER_WHITELIST, identifier_whitelist.contract_address
        );
    let identifier = OracleInterfaces::IDENTIFIER_WHITELIST;
    let time: u256 = 1000;
    let ancillary_data: ByteArray = Default::default();
    let ownable = IOwnableDispatcher { contract_address: oracle.contract_address };
    start_prank(CheatTarget::One(ownable.contract_address), OWNER());
    oracle.request_price(identifier, time, ancillary_data.clone());

    let price: u256 = 500;
    oracle_config.push_price(identifier, time, ancillary_data.clone(), price);
    let expected_event = mock_oracle_ancillary::Event::PushedPrice(
        mock_oracle_ancillary::PushedPrice {
            pusher: OWNER(),
            identifier: identifier,
            time: time,
            ancillary_data: ancillary_data.clone(),
            price: price,
            request_id: encode_price_request(identifier, time, @ancillary_data),
        }
    );
    spy.assert_emitted(@array![(oracle.contract_address, expected_event)]);

    assert(oracle.has_price(identifier, time, ancillary_data.clone()), 'Price should be available');
    assert_eq!(
        oracle.get_price(identifier, time, ancillary_data.clone()), price, "Price should match"
    );
    // // Test pushing a new price for the same request
    let new_price: u256 = 600;
    oracle_config.push_price(identifier, time, ancillary_data.clone(), new_price);
    assert_eq!(
        oracle.get_price(identifier, time, ancillary_data), new_price, "New price should be updated"
    );
}
#[test]
#[should_panic(expected: ('Price not requested',))]
fn test_push_price_not_requested() {
   let (finder, _) = setup_finder(); 
    let (_, oracle_config, _) = setup_mock_oracle_ancillary(finder);

    let identifier = OracleInterfaces::IDENTIFIER_WHITELIST; 
    let time: u256 = 1000;
    let ancillary_data: ByteArray = Default::default(); 
    let price: u256 = 500;
    let (identifier_whitelist, _) = setup_identifier_whitelist();
    let ownable = IOwnableDispatcher { contract_address: identifier_whitelist.contract_address };
    start_prank(CheatTarget::One(ownable.contract_address), OWNER());
    identifier_whitelist.add_supported_identifier(OracleInterfaces::IDENTIFIER_WHITELIST);
    let ownable = IOwnableDispatcher { contract_address: finder.contract_address };
    start_prank(CheatTarget::One(ownable.contract_address), OWNER());
    finder.change_implementation_address(OracleInterfaces::IDENTIFIER_WHITELIST, identifier_whitelist.contract_address); 

    oracle_config.push_price(identifier, time, ancillary_data, price);
}

#[test]
fn test_push_price_by_request_id() {
   let (finder, _) = setup_finder(); 
    let (oracle, oracle_config, mut spy) = setup_mock_oracle_ancillary(finder);
    let (identifier_whitelist, _) = setup_identifier_whitelist();
    let ownable = IOwnableDispatcher { contract_address: identifier_whitelist.contract_address };
    start_prank(CheatTarget::One(ownable.contract_address), OWNER());
    identifier_whitelist.add_supported_identifier(OracleInterfaces::IDENTIFIER_WHITELIST);
    let ownable = IOwnableDispatcher { contract_address: finder.contract_address };
    start_prank(CheatTarget::One(ownable.contract_address), OWNER());
    finder.change_implementation_address(OracleInterfaces::IDENTIFIER_WHITELIST, identifier_whitelist.contract_address); 
    let identifier = OracleInterfaces::IDENTIFIER_WHITELIST; 
    let time: u256 = 1000;
    let ancillary_data: ByteArray = Default::default(); 
    let ownable = IOwnableDispatcher { contract_address: oracle.contract_address };
    start_prank(CheatTarget::One(ownable.contract_address), OWNER());
    oracle.request_price(identifier, time, ancillary_data.clone());

    let request_id = encode_price_request(identifier, time, @ancillary_data);
    let price: u256 = 500;

    oracle_config.push_price_by_request_id(request_id, price);

    let expected_event = mock_oracle_ancillary::Event::PushedPrice(mock_oracle_ancillary::PushedPrice {
        pusher: OWNER(),
        identifier: identifier,
        time: time,
        ancillary_data: ancillary_data.clone(),
        price: price,
        request_id: request_id,
    });
    spy.assert_emitted(@array![(oracle.contract_address,expected_event )]);
    assert_eq!(oracle.get_price(identifier, time, ancillary_data), price, "Price should match");
}

#[test]
#[should_panic(expected: ('Request id not found',))]
fn test_push_price_by_invalid_request_id() {
   let (finder, _) = setup_finder(); 
    let (_, oracle_config, _) = setup_mock_oracle_ancillary(finder);
    let (identifier_whitelist, _) = setup_identifier_whitelist();
    let ownable = IOwnableDispatcher { contract_address: identifier_whitelist.contract_address };
    start_prank(CheatTarget::One(ownable.contract_address), OWNER());
    identifier_whitelist.add_supported_identifier(OracleInterfaces::IDENTIFIER_WHITELIST);
    let ownable = IOwnableDispatcher { contract_address: finder.contract_address };
    start_prank(CheatTarget::One(ownable.contract_address), OWNER());
    finder.change_implementation_address(OracleInterfaces::IDENTIFIER_WHITELIST, identifier_whitelist.contract_address); 

    let invalid_request_id: felt252 = 12345;
    let price: u256 = 500;

    oracle_config.push_price_by_request_id(invalid_request_id, price);
}

#[test]
fn test_get_pending_queries() {
   let (finder, _) = setup_finder(); 
    let (oracle, oracle_config, _) = setup_mock_oracle_ancillary(finder);
    let (identifier_whitelist, _) = setup_identifier_whitelist();
    let ownable = IOwnableDispatcher { contract_address: identifier_whitelist.contract_address };
    start_prank(CheatTarget::One(ownable.contract_address), OWNER());
    identifier_whitelist.add_supported_identifier(OracleInterfaces::IDENTIFIER_WHITELIST);
    let ownable = IOwnableDispatcher { contract_address: finder.contract_address };
    start_prank(CheatTarget::One(ownable.contract_address), OWNER());
    finder.change_implementation_address(OracleInterfaces::IDENTIFIER_WHITELIST, identifier_whitelist.contract_address); 
    let identifier1: felt252 = OracleInterfaces::IDENTIFIER_WHITELIST;
    let time1: u256 = 1000;
    let ancillary_data1: ByteArray = Default::default(); 
    oracle.request_price(identifier1, time1, ancillary_data1.clone());

    let identifier2: felt252 = OracleInterfaces::IDENTIFIER_WHITELIST;
    let time2: u256 = 2000;
    let ancillary_data2: ByteArray = Default::default(); 
    oracle.request_price(identifier2, time2, ancillary_data2.clone());

    let pending_queries: Span<mock_oracle_ancillary::QueryPoint> = oracle_config.get_pending_queries();

    assert_eq!(pending_queries.len(), 2, "Should have 2 pending queries");
    assert_eq!(*pending_queries.at(0).identifier, identifier1, "First query identifier mismatch");
    assert_eq!(*pending_queries.at(0).time, time1, "First query time mismatch");
    assert_eq!(*pending_queries.at(1).identifier, identifier2, "Second query identifier mismatch");
    assert_eq!(*pending_queries.at(1).time, time2, "Second query time mismatch");

    // Push price for the first query and check if it's removed from pending queries
    oracle_config.push_price(identifier1, time1, ancillary_data1, 500_u256);
    let updated_pending_queries = oracle_config.get_pending_queries();
    assert_eq!(updated_pending_queries.len(), 1, "Should have 1 pending query after push");
    assert_eq!(*updated_pending_queries.at(0).identifier, identifier2, "Remaining query identifier mismatch");
}

#[test]
fn test_get_request_parameters() {
   let (finder, _) = setup_finder(); 
    let (oracle, oracle_config, _) = setup_mock_oracle_ancillary(finder);
    let (identifier_whitelist, _) = setup_identifier_whitelist();
    let ownable = IOwnableDispatcher { contract_address: identifier_whitelist.contract_address };
    start_prank(CheatTarget::One(ownable.contract_address), OWNER());
    identifier_whitelist.add_supported_identifier(OracleInterfaces::IDENTIFIER_WHITELIST);
    let ownable = IOwnableDispatcher { contract_address: finder.contract_address };
    start_prank(CheatTarget::One(ownable.contract_address), OWNER());
    finder.change_implementation_address(OracleInterfaces::IDENTIFIER_WHITELIST, identifier_whitelist.contract_address); 
    let identifier = OracleInterfaces::IDENTIFIER_WHITELIST; 
    let time: u256 = 1000;
    let ancillary_data: ByteArray = Default::default(); 
    oracle.request_price(identifier, time, ancillary_data.clone());

    let request_id = encode_price_request(identifier, time, @ancillary_data);
    let query_point = oracle_config.get_request_parameters(request_id);

    assert_eq!(query_point.identifier, identifier, "Identifier mismatch");
    assert_eq!(query_point.time, time, "Time mismatch");
    assert_eq!(query_point.ancillary_data, ancillary_data, "Ancillary data mismatch");
}

#[test]
#[should_panic(expected: ('Request id not found',))]
fn test_get_request_parameters_invalid_id() {
   let (finder, _) = setup_finder(); 
    let (_, oracle_config, _) = setup_mock_oracle_ancillary(finder);
    let (identifier_whitelist, _) = setup_identifier_whitelist();
    let ownable = IOwnableDispatcher { contract_address: identifier_whitelist.contract_address };
    start_prank(CheatTarget::One(ownable.contract_address), OWNER());
    identifier_whitelist.add_supported_identifier(OracleInterfaces::IDENTIFIER_WHITELIST);
    let ownable = IOwnableDispatcher { contract_address: finder.contract_address };
    start_prank(CheatTarget::One(ownable.contract_address), OWNER());
    finder.change_implementation_address(OracleInterfaces::IDENTIFIER_WHITELIST, identifier_whitelist.contract_address); 
    let invalid_request_id: felt252 = 12345;
    oracle_config.get_request_parameters(invalid_request_id);
}

#[test]
fn test_has_price() {
   let (finder, _) = setup_finder(); 
    let (oracle, oracle_config, _) = setup_mock_oracle_ancillary(finder);
    let (identifier_whitelist, _) = setup_identifier_whitelist();
    let ownable = IOwnableDispatcher { contract_address: identifier_whitelist.contract_address };
    start_prank(CheatTarget::One(ownable.contract_address), OWNER());
    identifier_whitelist.add_supported_identifier(OracleInterfaces::IDENTIFIER_WHITELIST);
    let ownable = IOwnableDispatcher { contract_address: finder.contract_address };
    start_prank(CheatTarget::One(ownable.contract_address), OWNER());
    finder.change_implementation_address(OracleInterfaces::IDENTIFIER_WHITELIST, identifier_whitelist.contract_address); 
    let identifier = OracleInterfaces::IDENTIFIER_WHITELIST; 
    let time: u256 = 1000;
    let ancillary_data: ByteArray = Default::default(); 

    assert(!oracle.has_price(identifier, time, ancillary_data.clone()), 'Should not have price initially');

    oracle.request_price(identifier, time, ancillary_data.clone());
    assert(!oracle.has_price(identifier, time, ancillary_data.clone()), 'Price after request');

    oracle_config.push_price(identifier, time, ancillary_data.clone(), 500_u256);
    assert(oracle.has_price(identifier, time, ancillary_data), 'Should have price after push');
}

#[test]
#[should_panic(expected: ('Price not available',))]
fn test_get_price_not_available() {
   let (finder, _) = setup_finder(); 
    let (oracle, _, _) = setup_mock_oracle_ancillary(finder);
    let (identifier_whitelist, _) = setup_identifier_whitelist();
    let ownable = IOwnableDispatcher { contract_address: identifier_whitelist.contract_address };
    start_prank(CheatTarget::One(ownable.contract_address), OWNER());
    identifier_whitelist.add_supported_identifier(OracleInterfaces::IDENTIFIER_WHITELIST);
    let ownable = IOwnableDispatcher { contract_address: finder.contract_address };
    start_prank(CheatTarget::One(ownable.contract_address), OWNER());
    finder.change_implementation_address(OracleInterfaces::IDENTIFIER_WHITELIST, identifier_whitelist.contract_address); 
    let identifier = OracleInterfaces::IDENTIFIER_WHITELIST; 
    let time: u256 = 1000;
    let ancillary_data: ByteArray = Default::default(); 

    oracle.get_price(identifier, time, ancillary_data);
}

#[test]
fn test_price_request_lifecycle() {
   let (finder, _) = setup_finder(); 
    let (oracle, oracle_config, mut spy) = setup_mock_oracle_ancillary(finder);
    let (identifier_whitelist, _) = setup_identifier_whitelist();
    let ownable = IOwnableDispatcher { contract_address: identifier_whitelist.contract_address };
    start_prank(CheatTarget::One(ownable.contract_address), OWNER());
    identifier_whitelist.add_supported_identifier(OracleInterfaces::IDENTIFIER_WHITELIST);
    let ownable = IOwnableDispatcher { contract_address: finder.contract_address };
    start_prank(CheatTarget::One(ownable.contract_address), OWNER());
    finder.change_implementation_address(OracleInterfaces::IDENTIFIER_WHITELIST, identifier_whitelist.contract_address); 
    let identifier = OracleInterfaces::IDENTIFIER_WHITELIST; 
    let time: u256 = 1000;
    let ownable = IOwnableDispatcher { contract_address: oracle.contract_address };
    start_prank(CheatTarget::One(ownable.contract_address), OWNER());
    let mut ancillary_data: ByteArray = Default::default(); 

    // Step 1: Request price
    oracle.request_price(identifier, time, ancillary_data.clone());
    let expected_event = mock_oracle_ancillary::Event::PriceRequestAdded(mock_oracle_ancillary::PriceRequestAdded {
        requester: OWNER(),
        identifier: identifier,
        time: time,
        ancillary_data: ancillary_data.clone(),
        request_id: encode_price_request(identifier, time, @ancillary_data),
    });
    spy.assert_emitted(@array![(oracle.contract_address,expected_event )]);
    // Step 2: Check pending queries
    let pending_queries = oracle_config.get_pending_queries();
    assert_eq!(pending_queries.len(), 1, "Should have 1 pending query");

    // Step 3: Push price
    let price: u256 = 500;
    oracle_config.push_price(identifier, time, ancillary_data.clone(), price);
    let expected_event = mock_oracle_ancillary::Event::PushedPrice(mock_oracle_ancillary::PushedPrice {
        pusher: OWNER(),
        identifier: identifier,
        time: time,
        ancillary_data: ancillary_data.clone(),
        price: price,
        request_id: encode_price_request(identifier, time, @ancillary_data),
    });
    spy.assert_emitted(@array![(oracle.contract_address,expected_event )]);

    // Step 4: Verify price is available
    assert(oracle.has_price(identifier, time, ancillary_data.clone()), 'Price should be available');
    assert_eq!(oracle.get_price(identifier, time, ancillary_data.clone()), price, "Price should match");

    // Step 5: Check that pending queries is empty
    let updated_pending_queries = oracle_config.get_pending_queries();
    assert_eq!(updated_pending_queries.len(), 0, "Should have no pending queries");
    
    // Step 6: Try to push price again (should update the existing price)
    let new_price: u256 = 600;
    oracle_config.push_price(identifier, time, ancillary_data.clone(), new_price);
    assert_eq!(oracle.get_price(identifier, time, ancillary_data), new_price, "New price should be updated");
}


