pub mod ancillary_data {
    use starknet::ContractAddress;
    use core::byte_array::{ByteArray, ByteArrayTrait};
    use optimistic_oracle::contracts::utils::convert::{up_bytes, down_bytes};

    pub fn append_key_value_bytes_32(
        current_ancillary_data: ByteArray, key: ByteArray, value: u256
    ) -> ByteArray {
        let mut output: ByteArray = Default::default();
        let prefix: ByteArray = construct_prefix(@current_ancillary_data, key);
        output = ByteArrayTrait::concat(@current_ancillary_data, @prefix);
        output.append_word(up_bytes(value).try_into().unwrap(), 1);
        output.append_word(down_bytes(value).try_into().unwrap(), 31);
        output
    }
    pub fn append_key_value_felt252(
        current_ancillary_data: ByteArray, key: ByteArray, value: felt252
    ) -> ByteArray {
        let mut output: ByteArray = Default::default();
        let prefix: ByteArray = construct_prefix(@current_ancillary_data, key);
        output = ByteArrayTrait::concat(@current_ancillary_data, @prefix);
        output.append_word(value, 31);
        output
    }

    pub fn append_key_value_address(
        current_ancillary_data: ByteArray, key: ByteArray, value: ContractAddress
    ) -> ByteArray {
        let mut output: ByteArray = Default::default();
        let prefix: ByteArray = construct_prefix(@current_ancillary_data, key);
        output = ByteArrayTrait::concat(@current_ancillary_data, @prefix);
        output.append_word(value.into(), 31);
        output
    }


    pub fn construct_prefix(current_ancillary_data: @ByteArray, mut key: ByteArray) -> ByteArray {
        let mut output: ByteArray = Default::default();
        if (current_ancillary_data.len() > 0) {
            output.append_word(',', 1);
            output = ByteArrayTrait::concat(@output, @key);
            output.append_word(':', 1);
        } else {
            output = ByteArrayTrait::concat(@output, @key);
            output.append_word(':', 1);
        }
        output
    }
}
