pub fn bytes(input: &str) -> &[u8] {
    unsafe { input.as_bytes() }
}

pub fn string_bytes(input: &String) -> &[u8] {
    unsafe { input.as_bytes() }
}

/// Returns first the base and second the sig bits
pub fn split_off_sign_bits(base: u8, bit_count: u8) -> (u8, u8) {
    let mut mask = 0;
    for x in (8 - bit_count)..8 {
        mask |= 1 << x;
    }
    (base & (!mask), base & mask)
}

/// Returns first the base and second the sig bits
pub fn split_off_sign_bits_multi(base: &[u8], bit_count: u8) -> (Vec<u8>, Vec<u8>) {
    let mut mask = 0;
    for x in (8 - bit_count)..8 {
        mask |= 1 << x;
    }
    let mut result: (Vec<u8>, Vec<u8>) = (vec![0; base.len()], vec![0; base.len()]);
    for x in base.into_iter().enumerate() {
        result.0[x.0] = (*x.1) & (!mask);
        result.1[x.0] = (*x.1) & mask;
    }
    return result;
}

/// Returns first the base
pub fn split_off_sign_bits_lossy(base: u8, bit_count: u8) -> u8 {
    let mut mask = 0;
    for x in (8 - bit_count)..8 {
        mask |= 1 << x;
    }
    base & (!mask)
}

/// Returns first the base and second the sig bits
pub fn split_off_sign_bits_multi_lossy(base: &[u8], bit_count: u8) -> Vec<u8> {
    let mut mask = 0;
    for x in (8 - bit_count)..8 {
        mask |= 1 << x;
    }
    let mut result: Vec<u8> = vec![0; base.len()];
    for x in base.into_iter().enumerate() {
        result[x.0] = (*x.1) & (!mask);
    }
    return result;
}