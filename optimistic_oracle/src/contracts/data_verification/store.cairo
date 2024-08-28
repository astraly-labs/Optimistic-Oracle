#[starknet::contract]
pub mod store {
    use starknet::{ContractAddress, ClassHash};
    use optimistic_oracle::contracts::interfaces::{IStoreDispatcher, IStore, IStoreDispatcherTrait};
    component!(path: OwnableComponent, storage: ownable, event: OwnableEvent);
    use openzeppelin::token::erc20::interface::{ERC20ABIDispatcher, ERC20ABIDispatcherTrait};
    use optimistic_oracle::contracts::optimistic_oracle_v1::optimistic_oracle_v1::ETH_ADDRESS;
    use openzeppelin::access::ownable::OwnableComponent;
    use openzeppelin::upgrades::{interface::IUpgradeable, upgradeable::UpgradeableComponent};
    #[abi(embed_v0)]
    impl OwnableImpl = OwnableComponent::OwnableImpl<ContractState>;
    impl OwnableInternalImpl = OwnableComponent::InternalImpl<ContractState>;
    impl UpgradeableInternalImpl = UpgradeableComponent::InternalImpl<ContractState>;
    component!(path: UpgradeableComponent, storage: upgradeable, event: UpgradeableEvent);

    // THIS STORE CONTRACT ONLY IMPLEMENT THE CORE FUNCTIONALITIES WITH FLAT FEES. 

    #[storage]
    struct Storage {
        final_fee: LegacyMap::<ContractAddress, u256>,
        #[substorage(v0)]
        ownable: OwnableComponent::Storage,
        #[substorage(v0)]
        upgradeable: UpgradeableComponent::Storage,
    }


    #[derive(Drop, starknet::Event)]
    pub struct NewFinalFee {
        new_final_fee: u256
    }

    #[event]
    #[derive(Drop, starknet::Event)]
    enum Event {
        NewFinalFee: NewFinalFee,
        #[flat]
        OwnableEvent: OwnableComponent::Event,
        #[flat]
        UpgradeableEvent: UpgradeableComponent::Event,
    }

    pub mod Errors {
        pub const INSUFFICIENT_AMOUNT: felt252 = 'Insufficient amount';
        pub const INSUFFICIENT_ALLOWANCE: felt252 = 'Insufficient allowance';
    }


    #[constructor]
    fn constructor(ref self: ContractState, owner: ContractAddress) {
        self.ownable.initializer(owner);
    }

    #[abi(embed_v0)]
    impl Upgradeable of IUpgradeable<ContractState> {
        /// Upgrades the contract to a new implementation.
        /// Callable only by the owner
        /// # Arguments
        ///
        /// * `new_class_hash` - The class hash of the new implementation.
        fn upgrade(ref self: ContractState, new_class_hash: ClassHash) {
            self.ownable.assert_only_owner();
            self.upgradeable.upgrade(new_class_hash);
        }
    }

    #[abi(embed_v0)]
    impl IStoreImpl of IStore<ContractState> {
        fn pay_oracle_fees(self: @ContractState, erc20_address: ContractAddress, amount: u256) {
            let caller = starknet::get_caller_address();
            let contract_address = starknet::get_contract_address();
            let erc20 = ERC20ABIDispatcher { contract_address: erc20_address };
            assert(amount > 0, Errors::INSUFFICIENT_AMOUNT);
            assert(
                erc20.allowance(caller, contract_address) >= amount, Errors::INSUFFICIENT_ALLOWANCE
            );
            erc20.transfer_from(caller, contract_address, amount);
        }

        fn compute_final_fee(self: @ContractState, currency: ContractAddress) -> u256 {
            self.final_fee.read(currency)
        }

        fn set_final_fee(ref self: ContractState, currency: ContractAddress, new_final_fee: u256) {
            self.ownable.assert_only_owner();
            self.final_fee.write(currency, new_final_fee);
            self.emit(NewFinalFee { new_final_fee })
        }

        fn withdraw_funds(ref self: ContractState, receiver: ContractAddress) {
            self.ownable.assert_only_owner();
            let eth_dispatcher = ERC20ABIDispatcher {
                contract_address: ETH_ADDRESS // ETH Contract Address
                .try_into().unwrap()
            };
            let balance = eth_dispatcher.balanceOf(starknet::get_contract_address());
            eth_dispatcher.transfer(receiver, balance);
        }
    }
}

