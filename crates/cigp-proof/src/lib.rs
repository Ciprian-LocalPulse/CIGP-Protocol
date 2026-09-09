//! `cigp-proof` — construction and independent verification of CIGP
//! [`cigp_core::RoundProof`] objects.

pub mod build;
pub mod verify;

pub use build::{build_round_proof, compute_round_hash, RoundInputs};
pub use verify::{verify_round_proof, VerificationReport, VerifyError};
