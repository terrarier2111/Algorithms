const ALPHABET_START: u8 = 65; /// this is equal to `'A'` in UTF-8
const ALPHABET_END: u8 = 90;   /// this is equal to `'Z'` in UTF-8
const CHAR_MASK: u8 = !((1 << 5) | (1 << 6) | (1 << 7)); /// This mask eliminates the bits which are relevant for upper/lower case representation
// TODO: Support keeping the uppermost 3 bits in the output

/// Time complexity: O(n + a) // n being the number of input chars and a being the length of the key
/// Space complexity: O(1)
pub fn encrypt(input: &[u8], output: &mut [u8], key: &str) {
    perform_op(input, output, key, Box::new(|char_offset, key_offset| {
        let result_char = char_offset + key_offset;
        let mut result = ALPHABET_START + result_char;
        if result > ALPHABET_END { // go back by the entire length of the alphabet if we went OOB
            result -= (ALPHABET_END - ALPHABET_START) + 1;
        }
        result
    }));
}

/// Time complexity: O(n + a) // n being the number of input chars and a being the length of the key
/// Space complexity: O(1)
pub fn decrypt(input: &[u8], output: &mut [u8], key: &str) {
    perform_op(input, output, key, Box::new(|char_offset, key_offset| {
        let mut result = ALPHABET_START + char_offset - key_offset;
        if result < ALPHABET_START { // go forth by the entire length of the alphabet if we went OOB
            result += (ALPHABET_END - ALPHABET_START) + 1;
        }
        result
    }));
}

fn perform_op(input: &[u8], output: &mut [u8], key: &str, result_fn: Box<dyn Fn(u8, u8) -> u8>) {
    let key_offsets: Vec<u8> = key.as_bytes().iter().map(|x| (x & CHAR_MASK) - 1).collect(); /// Calculate the offset on one side of the table
    let key_offsets = key_offsets.as_slice();
    for x in input.iter().enumerate() {
        let key_offset = key_offsets[x.0 % key_offsets.len()];
        let char_offset = (x.1 & CHAR_MASK) - 1; /// Calculate the offset on the other side of the table
        let result = result_fn(char_offset, key_offset);
        output[x.0] = result; // `result` is the character at the specific x and y coordinate where our key and input at position x leads us to.
    }
}