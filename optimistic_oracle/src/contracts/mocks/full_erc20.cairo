#[starknet::contract]
pub mod full_erc20 {
    use openzeppelin::token::erc20::{ERC20Component, ERC20HooksEmptyImpl};
    use openzeppelin::introspection::src5::SRC5Component;
    use starknet::ContractAddress;
    use optimistic_oracle::contracts::interfaces::IExtendedERC20;
    use openzeppelin::access::accesscontrol::{AccessControlComponent, DEFAULT_ADMIN_ROLE};
    component!(path: SRC5Component, storage: src5, event: SRC5Event);
    component!(path: ERC20Component, storage: erc20, event: ERC20Event);
    component!(path: AccessControlComponent, storage: accesscontrol, event: AccessControlEvent);


    pub const MINTER_ROLE: felt252 = selector!("Minter");
    const OWNER_ROLE: felt252 = selector!("Owner");
    pub const BURNER_ROLE: felt252 = selector!("Burner");

    #[abi(embed_v0)]
    impl ERC20MixinImpl = ERC20Component::ERC20MixinImpl<ContractState>;
    impl InternalImpl = ERC20Component::InternalImpl<ContractState>;

    #[abi(embed_v0)]
    impl AccessControlMixinImpl =
        AccessControlComponent::AccessControlMixinImpl<ContractState>;
    impl AccessControlInternalImpl = AccessControlComponent::InternalImpl<ContractState>;

    #[storage]
    struct Storage {
        #[substorage(v0)]
        erc20: ERC20Component::Storage,
        #[substorage(v0)]
        accesscontrol: AccessControlComponent::Storage,
        #[substorage(v0)]
        src5: SRC5Component::Storage
    }

    #[event]
    #[derive(Drop, starknet::Event)]
    enum Event {
        #[flat]
        ERC20Event: ERC20Component::Event,
        #[flat]
        AccessControlEvent: AccessControlComponent::Event,
        #[flat]
        SRC5Event: SRC5Component::Event
    }

    #[constructor]
    fn constructor(
        ref self: ContractState, name: ByteArray, symbol: ByteArray, owner: ContractAddress
    ) {
        self.erc20.initializer(name, symbol);
        self.accesscontrol.initializer();
        self.accesscontrol._grant_role(OWNER_ROLE, owner);
    }

    #[abi(embed_v0)]
    impl IExtendedERC20Impl of IExtendedERC20<ContractState> {
        fn mint(ref self: ContractState, recipient: ContractAddress, value: u256) {
            let is_minter = self
                .accesscontrol
                .has_role(MINTER_ROLE, starknet::get_caller_address());
            assert(is_minter, 'Only Minter can mint');
            self.erc20.mint(recipient, value);
        }

        fn burn(ref self: ContractState, account: ContractAddress, value: u256) {
            let is_burner = self
                .accesscontrol
                .has_role(BURNER_ROLE, starknet::get_caller_address());
            assert(is_burner, 'Only Burner can burn');
            self.erc20.mint(account, value);
        }

        fn only_owner(self: @ContractState, caller_address: ContractAddress) -> bool {
            self.accesscontrol.has_role(OWNER_ROLE, caller_address)
        }

        fn grant_minter_role(ref self: ContractState, minter: ContractAddress) {
            let is_owner = self.accesscontrol.has_role(OWNER_ROLE, starknet::get_caller_address());
            assert!(is_owner, "Only Owner can grant minter role");
            self.accesscontrol._grant_role(MINTER_ROLE, minter);
        }

        fn revoke_minter_role(ref self: ContractState, account: ContractAddress) {
            let is_owner = self.accesscontrol.has_role(OWNER_ROLE, starknet::get_caller_address());
            assert!(is_owner, "Only Owner can revoke minter role");
            self.accesscontrol._revoke_role(MINTER_ROLE, account);
        }


        fn grant_burner_role(ref self: ContractState, burner: ContractAddress) {
            let is_owner = self.accesscontrol.has_role(OWNER_ROLE, starknet::get_caller_address());
            assert!(is_owner, "Only Owner can grant burner role");
            self.accesscontrol._grant_role(BURNER_ROLE, burner);
        }

        fn revoke_burner_role(ref self: ContractState, account: ContractAddress) {
            let is_owner = self.accesscontrol.has_role(OWNER_ROLE, starknet::get_caller_address());
            assert!(is_owner, "Only Owner can revoke burner role");
            self.accesscontrol._revoke_role(BURNER_ROLE, account);
        }
    }
}
