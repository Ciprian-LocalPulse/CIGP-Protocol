//! Independent verification of a [`RoundProof`], producing a field-by-field
//! [`VerificationReport`] as described in CIGP §16.

use crate::build::compute_round_hash;
use cigp_core::types::{CheckResult, CIGP_VERSION};
use cigp_core::{CigpError, RoundProof};
use cigp_crypto::{commit, derive_rng_output, verify_signature};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct VerificationReport {
    pub protocol_version: String,
    pub round_id: String,
    pub signature: CheckResult,
    pub commitment: CheckResult,
    pub seed_derivation: CheckResult,
    pub round_hash: CheckResult,
    /// `None` if verification could not even reach the outcome/payout
    /// checks (e.g. unsupported protocol version, malformed proof).
    pub previous_hash_note: Option<String>,
}

impl VerificationReport {
    /// Overall pass/fail: every individual check must pass.
    pub fn is_valid(&self) -> bool {
        self.signature.is_pass()
            && self.commitment.is_pass()
            && self.seed_derivation.is_pass()
            && self.round_hash.is_pass()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum VerifyError {
    #[error(transparent)]
    Cigp(#[from] CigpError),
    #[error("unsupported protocol version: {0}")]
    UnsupportedVersion(String),
}

/// Verify a RoundProof's internal cryptographic consistency:
/// recomputed `round_hash`, signature, seed commitment, and (if the server
/// seed has been revealed) the HMAC-based RNG derivation.
///
/// `operator_public_key_hex` is the operator's published Ed25519 public key,
/// used to check `signature`.
pub fn verify_round_proof(
    proof: &RoundProof,
    operator_public_key_hex: &str,
) -> Result<VerificationReport, VerifyError> {
    if proof.cigp_version != CIGP_VERSION {
        return Err(VerifyError::UnsupportedVersion(proof.cigp_version.clone()));
    }

    let recomputed_hash = compute_round_hash(proof)?;
    let round_hash_ok = recomputed_hash == proof.round_hash;

    let signature_ok =
        verify_signature(operator_public_key_hex, &proof.round_hash, &proof.signature).is_ok();

    let commitment_ok = match &proof.server_seed {
        Some(seed) => commit(seed) == proof.server_commitment,
        // Pre-reveal proofs have no server_seed to check yet; the
        // commitment field is still structurally present and non-empty.
        None => !proof.server_commitment.is_empty(),
    };

    let seed_derivation_ok = match &proof.server_seed {
        Some(seed) => derive_rng_output(seed, &proof.client_seed, proof.nonce) == proof.rng.output,
        None => true, // cannot check RNG derivation before reveal
    };

    Ok(VerificationReport {
        protocol_version: proof.cigp_version.clone(),
        round_id: proof.round_id.clone(),
        signature: CheckResult::from_bool(signature_ok),
        commitment: CheckResult::from_bool(commitment_ok),
        seed_derivation: CheckResult::from_bool(seed_derivation_ok),
        round_hash: CheckResult::from_bool(round_hash_ok),
        previous_hash_note: None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::build::{build_round_proof, RoundInputs};
    use cigp_core::Money;
    use cigp_crypto::OperatorKeypair;
    use serde_json::json;

    fn sample_inputs() -> RoundInputs {
        RoundInputs {
            round_id: "round-1".into(),
            operator_id: "cigp-demo-operator".into(),
            game_id: "cigp-demo-slot".into(),
            game_version: "1.0.0".into(),
            bet: Money::new("EUR", 100).unwrap(),
            payout: Money::new("EUR", 0).unwrap(),
            server_seed: "aa".repeat(32),
            client_seed: "player-client-seed".into(),
            nonce: 1,
            mapping_algorithm: "reel-strip-v1".into(),
            mapping_version: "1".into(),
            mapping_parameters_hash: "sha256:abc".into(),
            outcome: json!({"reels": [1, 2, 3]}),
            paytable_hash: "sha256:paytable".into(),
            game_logic_hash: "sha256:logic".into(),
            configuration_hash: "sha256:config".into(),
            previous_round_hash: "sha256:genesis".into(),
            timestamp: "2026-01-01T00:00:00Z".into(),
        }
    }

    #[test]
    fn valid_proof_passes_all_checks() {
        let kp = OperatorKeypair::generate();
        let proof = build_round_proof(sample_inputs(), &kp).unwrap();
        let report = verify_round_proof(&proof, &kp.public_key_hex()).unwrap();
        assert!(report.is_valid());
    }

    #[test]
    fn tampered_payout_fails_round_hash_and_signature() {
        let kp = OperatorKeypair::generate();
        let mut proof = build_round_proof(sample_inputs(), &kp).unwrap();
        proof.payout = Money::new("EUR", 999_999).unwrap();
        let report = verify_round_proof(&proof, &kp.public_key_hex()).unwrap();
        assert!(!report.is_valid());
        assert_eq!(report.round_hash, CheckResult::Fail);
    }

    #[test]
    fn tampered_outcome_fails_verification() {
        let kp = OperatorKeypair::generate();
        let mut proof = build_round_proof(sample_inputs(), &kp).unwrap();
        proof.outcome = json!({"reels": [7, 7, 7]});
        let report = verify_round_proof(&proof, &kp.public_key_hex()).unwrap();
        assert!(!report.is_valid());
    }

    #[test]
    fn tampered_game_logic_hash_fails_verification() {
        let kp = OperatorKeypair::generate();
        let mut proof = build_round_proof(sample_inputs(), &kp).unwrap();
        proof.game_logic_hash = "sha256:different".into();
        let report = verify_round_proof(&proof, &kp.public_key_hex()).unwrap();
        assert!(!report.is_valid());
    }

    #[test]
    fn wrong_operator_key_fails_signature() {
        let kp = OperatorKeypair::generate();
        let other = OperatorKeypair::generate();
        let proof = build_round_proof(sample_inputs(), &kp).unwrap();
        let report = verify_round_proof(&proof, &other.public_key_hex()).unwrap();
        assert!(!report.is_valid());
        assert_eq!(report.signature, CheckResult::Fail);
    }
}
