use core::byte_array::{ByteArray, ByteArrayTrait};
use starknet::ContractAddress;


#[starknet::interface]
pub trait IOptimisticOracle<TContractState> {
    fn assert_truth(ref self: TContractState, claim: ByteArray, asserter: ContractAddress, callback_recipient: ContractAddress, escalation_manager: ContractAddress);
}   

#[starknet::interface]
pub trait IFinder<TContractState> {
    fn change_implementation_address(ref self: TContractState, interface_name: felt252 , implementation_address: ContractAddress); 

    fn get_implementation_address(self: @TContractState, interface_name: felt252 ) -> ContractAddress;
}