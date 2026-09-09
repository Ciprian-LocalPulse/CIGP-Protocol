//! `cigp-core` — protocol-versioned domain types and canonical serialization
//! for the Casino Integrity & Gaming Proof Protocol (CIGP).
//!
//! This crate is deliberately free of any I/O, RNG, or key-management
//! concerns. It defines *what a RoundProof is*, not how one is produced or
//! signed — see `cigp-crypto` and `cigp-proof` for that.

pub mod canonical;
pub mod error;
pub mod money;
pub mod types;

pub use error::CigpError;
pub use money::Money;
pub use types::{
    CheckResult, GameManifest, HashRef, Mathematics, MappingEvidence, RngEvidence, RngProfile,
    RoundProof, CIGP_VERSION, REFERENCE_RNG_PROFILE,
};
