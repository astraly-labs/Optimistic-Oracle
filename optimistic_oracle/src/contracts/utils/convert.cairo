pub const BYTES_IN_BYTES31: usize = 31;


pub fn convert_byte_array_to_felt_array(input: @ByteArray) -> Array<felt252> {
    let mut output: Array<felt252> = ArrayTrait::new();
    let mut current_word: felt252 = 0;
    let mut bytes_in_current_word: usize = 0;
    let input_len = input.len();

    let mut i: usize = 0;
    loop {
        if i >= input_len {
            break;
        }

        if let Option::Some(byte) = input.at(i) {
            current_word = current_word * 256 + byte.into();
            bytes_in_current_word += 1;

            if bytes_in_current_word == BYTES_IN_BYTES31 {
                output.append(current_word.try_into().unwrap());
                current_word = 0;
                bytes_in_current_word = 0;
            }
        }

        i += 1;
    };

    // Handle any remaining bytes
    if bytes_in_current_word > 0 {
        // Pad the last word with zeros if necessary
        while bytes_in_current_word < BYTES_IN_BYTES31 {
            current_word = current_word * 256;
            bytes_in_current_word += 1;
        };
        output.append(current_word.try_into().unwrap());
    }

    output
}
