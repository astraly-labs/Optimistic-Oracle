#[starknet::contract]
pub mod finder {
    use core::starknet::event::EventEmitter;
use optimistic_oracle::contracts::interfaces::IFinder;
    use openzeppelin::access::ownable::OwnableComponent;
    use starknet::{ContractAddress,contract_address_const};
    component!(path: OwnableComponent, storage: ownable, event: OwnableEvent);
    #[abi(embed_v0)]
    impl OwnableImpl = OwnableComponent::OwnableImpl<ContractState>;
    impl OwnableInternalImpl = OwnableComponent::InternalImpl<ContractState>;


    #[storage]
    struct Storage{
        interface_implemented: LegacyMap::<felt252, ContractAddress>,
        #[substorage(v0)]
        ownable: OwnableComponent::Storage,
    }

    
    #[event]
    #[derive(Drop, starknet::Event)]
    enum Event {
        InterfaceImplementationChanged:InterfaceImplementationChanged,
        #[flat]
        OwnableEvent: OwnableComponent::Event,
    }

    pub mod Errors {
        pub const IMPLEMENTATION_NOT_FOUND: felt252 = 'Implementation not found';
    }


    #[derive(starknet::Event, Drop)]
    pub struct InterfaceImplementationChanged {
        pub interface_name: felt252, 
        pub new_implementation_address: ContractAddress,
    }

    #[constructor]
    fn constructor(ref self: ContractState, owner: ContractAddress) {
        self.ownable.initializer(owner);
    }

    #[abi(embed_v0)]
    impl IFinderImpl of IFinder<ContractState> {
        fn change_implementation_address(ref self: ContractState, interface_name: felt252, implementation_address:ContractAddress){
            self.ownable.assert_only_owner();
            self.interface_implemented.write(interface_name, implementation_address);
            self.emit(
                InterfaceImplementationChanged{
                    interface_name: interface_name, 
                    new_implementation_address: implementation_address,
                }
            );
        }

        fn get_implementation_address(self: @ContractState, interface_name: felt252) -> ContractAddress {
            let implementation_address = self.interface_implemented.read(interface_name);
            assert(implementation_address !=contract_address_const::<0>(), Errors::IMPLEMENTATION_NOT_FOUND);
            implementation_address
        }


    }

}