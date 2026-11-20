// HINT: `sha2::{Digest, Sha256}` is already a dependency — hash to bytes, then hex-encode.
pub fn hash_password(password: &str) -> String {
    todo!("SHA-256 hash password and return a hex string")
}

pub fn verify_password(password: &str, hash: &str) -> bool {
    todo!("compare hash_password(password) with hash")
}

// HINT: XOR each byte with a repeating key, then hex-encode the result.
pub fn encrypt(plain: &str, key: &str) -> String {
    todo!("XOR plain against key bytes and return hex")
}

// HINT: hex-decode first, XOR, then String::from_utf8.
pub fn decrypt(cipher: &str, key: &str) -> Result<String, String> {
    todo!("reverse encrypt()")
}

fn to_hex(bytes: &[u8]) -> String {
    todo!("format each byte as two lowercase hex digits")
}

fn from_hex(text: &str) -> Result<Vec<u8>, String> {
    todo!("parse each two-character chunk with u8::from_str_radix(..., 16)")
}
