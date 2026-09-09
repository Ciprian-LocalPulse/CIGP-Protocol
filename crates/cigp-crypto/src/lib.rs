//! `cigp-crypto` — cryptographic primitives for CIGP: SHA-256 hashing,
//! the reference commit-reveal / HMAC-SHA256 RNG derivation, Ed25519
//! signatures, and Merkle tree batching with inclusion proofs.
//!
//! CIGP v0.1 cryptographic profile:
//!
//! | Purpose             | Primitive        |
//! |----------------------|------------------|
//! | Content hashing      | SHA-256          |
//! | Seed derivation MAC   | HMAC-SHA-256     |
//! | Key derivation        | HKDF-SHA256      |
//! | Digital signatures    | Ed25519          |
//! | Integrity structure   | Hash chain + Merkle tree |
//!
//! No cryptographic algorithm is invented here; all primitives come from
//! well-maintained RustCrypto / dalek-cryptography crates.

pub mod commit_reveal;
pub mod hashing;
pub mod merkle;
pub mod signature;

pub use commit_reveal::{commit, derive_rng_output, generate_server_seed, hkdf_expand, verify_commitment};
pub use hashing::{sha256_bytes, sha256_hex, sha256_hex_raw};
pub use merkle::{MerkleError, MerkleTree, ProofStep};
pub use signature::{verify as verify_signature, OperatorKeypair, SignatureError};
