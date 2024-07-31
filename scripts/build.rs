use std::{collections::HashMap, env::current_dir, path::PathBuf};

use ethers::prelude::Abigen;

fn generate_strk_bind(name: &str, abi_file: &str, bind_out: PathBuf) {
    if bind_out.exists() {
        std::fs::remove_file(&bind_out).unwrap();
    }

    let mut aliases = HashMap::new();
    aliases.insert(
        String::from("openzeppelin::access::ownable::ownable::OwnableComponent::Event"),
        String::from("OwnableCptEvent"),
    );
    aliases.insert(
        String::from("openzeppelin::upgrades::upgradeable::UpgradeableComponent::Event"),
        String::from("UpgradeableCptEvent"),
    );
    aliases.insert(
        String::from("openzeppelin::security::reentrancyguard::::ReentrancyGuardComponent::Event"),
        String::from("ReentrancyGuardCptEvent"),
    );
    let abigen = cainome::rs::Abigen::new(name, abi_file).with_types_aliases(aliases);

    abigen
        .generate()
        .expect("Fail to generate bindings")
        .write_to_file(&bind_out.to_str().expect("valid utf8 path"))
        .expect("Fail to write bindings to file");
}

fn main() {
    let abi_base = current_dir()
        .unwrap()
        .parent()
        .unwrap()
        .join("optimistic_oracle/target/dev");
    let bind_base = current_dir().unwrap().join("src/bind");
    let deployments = [
        ("finder", "finder"),
        ("address_whitelist", "address_whitelist"),
        ("identifier_whitelist", "identifier_whitelist"),
        ("store", "store"),
        ("mock_oracle_ancillary", "mock_oracle_ancillary"),
        ("optimistic_oracle_v1", "optimistic_oracle_v1"),
    ];

    for (abi_file, bind_out) in deployments {
        generate_strk_bind(
            abi_file,
            abi_base
                .join(format!("optimistic_oracle_{abi_file}.contract_class.json"))
                .to_str()
                .unwrap(),
            bind_base.join(format!("{bind_out}.rs")),
        );
    }
}
