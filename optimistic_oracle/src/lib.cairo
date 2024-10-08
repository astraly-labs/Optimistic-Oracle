pub mod contracts {
    pub mod interfaces;
    pub mod optimistic_oracle_v1;
    pub mod finder;
    pub mod utils {
        pub mod constants;
        pub mod convert;
        pub mod ancillary_data;
        pub mod keccak;
    }
    pub mod data_verification {
        pub mod identifier_whitelist;
        pub mod store;
    }
    pub mod escalation_manager {
        pub mod base_escalation_manager;
        pub mod whitelisted_escalation_manager;
    }
    pub mod common {
        pub mod address_whitelist;
    }
    pub mod mocks {
        pub mod oracle_ancillary;
        pub mod full_erc20;
        pub mod mock_erc20;
        pub mod mock_oracle;
    }
}
pub mod examples {
    pub mod prediction_market;
}

#[cfg(test)]
pub mod tests {
    pub mod setup;
    pub mod test_finder;
    pub mod test_address_whitelist;
    pub mod test_identifier_whitelist;
    pub mod test_oracle_ancillary;
    pub mod test_optimistic_oracle;
}
