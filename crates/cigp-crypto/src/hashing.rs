//! SHA-256 hashing helpers.
//!
//! CIGP uses plain SHA-256 only for commitments and content fingerprints
//! (`server_commitment`, `*_hash` fields), never as a substitute for HMAC in
//! contexts requiring key-dependent authentication. See [`crate::hmac_derive`]
//! for that.

use sha2::{Digest, Sha256};

/// SHA-256 of raw bytes, returned as a `sha256:<hex>` tagged string.
pub fn sha256_hex(data: &[u8]) -> String {
    let digest = Sha256::digest(data);
    format!("sha256:{}", hex::encode(digest))
}

/// SHA-256 of raw bytes, returned as untagged lowercase hex.
pub fn sha256_hex_raw(data: &[u8]) -> String {
    hex::encode(Sha256::digest(data))
}

/// SHA-256 of raw bytes, returned as raw digest bytes.
pub fn sha256_bytes(data: &[u8]) -> [u8; 32] {
    let digest = Sha256::digest(data);
    let mut out = [0u8; 32];
    out.copy_from_slice(&digest);
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_vector() {
        // SHA-256("") = e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
        assert_eq!(
            sha256_hex_raw(b""),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
    }
}
