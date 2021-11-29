const MASK: u8 = !((1 << 5) | (1 << 6) | (1 << 7)); // TODO: Use this!

/// Time complexity: O(n)
/// Space complexity: O(1)
pub fn encrypt(input: &[u8], output: &mut [u8], shift_num: u8) {
    // 65 - 90
    for x in input.iter().enumerate() {
        output[x.0] = shift(*x.1,shift_num);
    }
}

/// Time complexity: O(n)
/// Space complexity: O(1)
pub fn decrypt(input: &[u8], output: &mut [u8], shift_num: u8) {
    // 65 - 90
    for x in input.iter().enumerate() {
        output[x.0] = unshift(*x.1,shift_num);
    }
}

/// Time complexity: O(1)
/// Space complexity: O(1)
fn shift(x: u8, offset: u8) -> u8 {
    if x + offset > 90 {
        return x + offset - 90 + 65;
    }
    return x + offset;
}

/// Time complexity: O(1)
/// Space complexity: O(1)
fn unshift(x: u8, offset: u8) -> u8 {
    let actual_char = (x as i16) - (offset as i16);
    if actual_char < 65 {
        return (actual_char + 25) as u8;
    }
    return actual_char as u8;
}
