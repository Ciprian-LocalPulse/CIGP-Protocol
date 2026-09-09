//! Reference commit-reveal protocol (CIGP §10) and the reference RNG
//! derivation (`CIGP-REFERENCE-HMAC-SHA256`).
//!
//! This is *the CIGP reference proof construction*, not a universal
//! requirement imposed on every certified gaming platform — other
//! cryptographic profiles may be registered under a different `rng.profile`
//! identifier. See `spec/cryptographic-profile.md`.

use hmac::{Hmac, Mac};
use rand_core::RngCore;
use sha2::Sha256;

use crate::hashing::sha256_hex_raw;

type HmacSha256 = Hmac<Sha256>;

/// Generate a fresh, cryptographically random 32-byte server seed, hex-encoded.
pub fn generate_server_seed() -> String {
    let mut seed = [0u8; 32];
    rand::rngs::OsRng.fill_bytes(&mut seed);
    hex::encode(seed)
}

/// Compute the public commitment for a server seed: `sha256(server_seed)`.
///
/// The server seed itself MUST NOT be revealed until the reveal phase; only
/// this commitment is published before the round is played.
pub fn commit(server_seed_hex: &str) -> String {
    sha256_hex_raw(server_seed_hex.as_bytes())
}

/// Verify that a revealed server seed matches a previously published
/// commitment.
pub fn verify_commitment(server_seed_hex: &str, commitment_hex: &str) -> bool {
    // Constant-time-ish comparison via hex re-derivation is unnecessary here
    // since the commitment is public data, not a secret being compared
    // against an authenticator; a straightforward equality check suffices.
    commit(server_seed_hex) == commitment_hex
}

/// Derive the reference RNG output for a round:
/// `HMAC-SHA256(server_seed, canonical(client_seed || nonce))`.
///
/// Returns the raw HMAC output as lowercase hex.
pub fn derive_rng_output(server_seed_hex: &str, client_seed: &str, nonce: u64) -> String {
    let key = hex::decode(server_seed_hex).unwrap_or_else(|_| server_seed_hex.as_bytes().to_vec());
    let mut mac = HmacSha256::new_from_slice(&key).expect("HMAC accepts any key length");
    let message = canonical_message(client_seed, nonce);
    mac.update(&message);
    let result = mac.finalize().into_bytes();
    hex::encode(result)
}

/// Canonical message bytes for `client_seed || nonce`, using a explicit
/// separator and fixed-width nonce encoding to avoid ambiguity (e.g.
/// `"seedA" || 12` colliding with `"seedA1" || 2`).
fn canonical_message(client_seed: &str, nonce: u64) -> Vec<u8> {
    let mut out = Vec::new();
    out.extend_from_slice(client_seed.as_bytes());
    out.push(0x1f); // ASCII unit separator, disambiguates the concatenation
    out.extend_from_slice(&nonce.to_be_bytes());
    out
}

/// Derive a stream of pseudo-random bytes of arbitrary length from RNG
/// evidence, using HKDF-SHA256, for games whose outcome mapping needs more
/// entropy than a single 32-byte HMAC output (e.g. multiple reel positions).
pub fn hkdf_expand(rng_output_hex: &str, info: &[u8], out_len: usize) -> Vec<u8> {
    let ikm = hex::decode(rng_output_hex).unwrap_or_default();
    let hk = hkdf::Hkdf::<Sha256>::new(None, &ikm);
    let mut okm = vec![0u8; out_len];
    hk.expand(info, &mut okm)
        .expect("HKDF output length must be <= 255 * hash length");
    okm
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn commitment_round_trips() {
        let seed = generate_server_seed();
        let commitment = commit(&seed);
        assert!(verify_commitment(&seed, &commitment));
        assert!(!verify_commitment(&seed, &commit("different")));
    }

    #[test]
    fn derivation_is_deterministic() {
        let seed = "aa".repeat(32);
        let a = derive_rng_output(&seed, "client-seed", 7);
        let b = derive_rng_output(&seed, "client-seed", 7);
        assert_eq!(a, b);
    }

    #[test]
    fn derivation_changes_with_nonce() {
        let seed = "aa".repeat(32);
        let a = derive_rng_output(&seed, "client-seed", 7);
        let b = derive_rng_output(&seed, "client-seed", 8);
        assert_ne!(a, b);
    }

    #[test]
    fn derivation_changes_with_client_seed() {
        let seed = "aa".repeat(32);
        let a = derive_rng_output(&seed, "client-seed-a", 7);
        let b = derive_rng_output(&seed, "client-seed-b", 7);
        assert_ne!(a, b);
    }
}
