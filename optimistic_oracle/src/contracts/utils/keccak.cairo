

const KECCAK_FULL_RATE_IN_U64S: usize = 17;
use core::starknet::SyscallResultTrait;
/// The padding in keccak256 is "1 0* 1".
/// `last_input_num_bytes` (0-7) is the number of bytes in the last u64 input - `last_input_word`.
fn add_padding(ref input: Array<u64>, last_input_word: u64, last_input_num_bytes: usize) {
    let words_divisor = KECCAK_FULL_RATE_IN_U64S.try_into().unwrap();
    // `last_block_num_full_words` is in range [0, KECCAK_FULL_RATE_IN_U64S - 1]
    let (_, last_block_num_full_words) = core::integer::u32_safe_divmod(input.len(), words_divisor);

    // The first word to append would be of the form
    //     0x1<`last_input_num_bytes` LSB bytes of `last_input_word`>.
    // For example, for `last_input_num_bytes == 4`:
    //     0x1000000 + (last_input_word & 0xffffff)
    let first_word_to_append = if last_input_num_bytes == 0 {
        // This case is handled separately to avoid unnecessary computations.
        1
    } else {
        let first_padding_byte_part = if last_input_num_bytes == 1 {
            0x100
        } else if last_input_num_bytes == 2 {
            0x10000
        } else if last_input_num_bytes == 3 {
            0x1000000
        } else if last_input_num_bytes == 4 {
            0x100000000
        } else if last_input_num_bytes == 5 {
            0x10000000000
        } else if last_input_num_bytes == 6 {
            0x1000000000000
        } else if last_input_num_bytes == 7 {
            0x100000000000000
        } else {
            core::panic_with_felt252('Keccak last input word >7b')
        };
        let (_, r) = core::integer::u64_safe_divmod(
            last_input_word, first_padding_byte_part.try_into().unwrap()
        );
        first_padding_byte_part + r
    };

    if last_block_num_full_words == KECCAK_FULL_RATE_IN_U64S - 1 {
        input.append(0x8000000000000000 + first_word_to_append);
        return;
    }

    // last_block_num_full_words < KECCAK_FULL_RATE_IN_U64S - 1
    input.append(first_word_to_append);
    finalize_padding(ref input, KECCAK_FULL_RATE_IN_U64S - 1 - last_block_num_full_words);
}

/// Finalize the padding by appending "0* 1".
fn finalize_padding(ref input: Array<u64>, num_padding_words: u32) {
    if (num_padding_words == 1) {
        input.append(0x8000000000000000);
        return;
    }

    input.append(0);
    finalize_padding(ref input, num_padding_words - 1);
}

/// Computes the Keccak hash of the input ByteArray.
///
/// Returns the hash as a little endian u256.
pub fn compute_keccak_byte_array(arr: @ByteArray) -> u256 {
    let mut input = array![];
    let mut i = 0;
    let mut inner = 0;
    let mut limb: u64 = 0;
    let mut factor: u64 = 1;
    while let Option::Some(b) = arr.at(i) {
        limb = limb + b.into() * factor;
        i += 1;
        inner += 1;
        if inner == 8 {
            input.append(limb);
            inner = 0;
            limb = 0;
            factor = 1;
        } else {
            factor *= 0x100;
        }
    };
    add_padding(ref input, limb, inner);
    starknet::syscalls::keccak_syscall(input.span()).unwrap_syscall()
}