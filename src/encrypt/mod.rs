pub mod ceasar;
pub mod polyalphabet;

pub fn encrypt_caesar(input: &str, offset: u8) -> String {
    let mut out = input.to_string();
    let mut out = unsafe { out.as_bytes_mut() };
    ceasar::encrypt(input.as_bytes(), out, offset);
    (*String::from_utf8_lossy(out)).to_string()
}

pub fn encrypt_poly(input: &str, key: &str) -> String {
    let mut out = input.to_string();
    let mut out = unsafe { out.as_bytes_mut() };
    polyalphabet::encrypt(input.as_bytes(), out, key);
    (*String::from_utf8_lossy(out)).to_string()
}

pub fn decrypt_caesar(input: &str, offset: u8) -> String {
    let mut out = input.to_string();
    let mut out = unsafe { out.as_bytes_mut() };
    ceasar::decrypt(input.as_bytes(), out, offset);
    (*String::from_utf8_lossy(out)).to_string()
}

pub fn decrypt_poly(input: &str, key: &str) -> String {
    let mut out = input.to_string();
    let mut out = unsafe { out.as_bytes_mut() };
    polyalphabet::decrypt(input.as_bytes(), out, key);
    (*String::from_utf8_lossy(out)).to_string()
}