use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::ops::Range;
use crate::break_encrypt::BreakError;
use crate::encrypt::ceasar::decrypt;
use crate::encrypt::encrypt_caesar;

/// Time complexity: O(n * a) // n being the number of input chars and a being the length of the alphabet
/// Space complexity: O(1)
pub fn break_caesar(input: &str, expected: &str) -> Result<u8, BreakError> {
    let inpt = input.as_bytes();
    let expected_result = expected.as_bytes();

    /// Create a buffer to which we write our cipher output later.
    let mut buffer = input.to_string();
    let buffer = unsafe { buffer.as_bytes_mut() };

    for x in 0..26 { /// Iterate over the number of letters in the alphabet and check for every possible shift
        decrypt(inpt, buffer, x);
        if buffer == expected_result { /// Compare the result with the expected result
                                       /// we could also add this to a list which we could return later
                                       /// and which contains all possible permutations.
            return Ok(x);
        }
    }
    /// This means that we didn't find the expected result here!
    return Err(BreakError {
        msg: "".to_string()
    });
}

/// Time complexity: O(n * a) // n being the number of input chars and a being the length of the alphabet
/// in this case a = 26
/// Space complexity: O(a)
pub fn break_caesar_permuts_default(input: &str) -> Vec<String> {
    break_caesar_permuts(input, 0..26) // Iterate over the number of letters in the alphabet and check for every possible shift
}

/// Time complexity: O(n * a) // n being the number of input chars and a being the length of the alphabet
/// in this case a = 26
/// Space complexity: O(a)
pub fn break_caesar_permuts(input: &str, range: Range<u8>) -> Vec<String> {
    let inpt = input.as_bytes();
    let mut result = vec![];
    for x in range { /// Iterate over the given range and check for every possible shift
        /// Create a buffer to which we write our cipher output later.
        let mut buffer = input.to_string();
        let buffer = unsafe { buffer.as_bytes_mut() };
        decrypt(inpt, buffer, x);
        result.push(String::from_utf8_lossy(buffer).to_string());
    }
    return result;
}

