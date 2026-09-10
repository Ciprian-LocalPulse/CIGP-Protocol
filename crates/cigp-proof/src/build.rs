//! Construction of a signed [`RoundProof`] from round inputs.

use cigp_core::canonical::canonical_hash;
use cigp_core::types::{MappingEvidence, RngEvidence, CIGP_VERSION};
use cigp_core::{CigpError, Money, RoundProof};
use cigp_crypto::{commit, derive_rng_output, OperatorKeypair};
use serde::Serialize;

/// Fields the round hash is computed over: every RoundProof field *except*
/// `round_hash` and `signature` themselves, since those are derived from
/// and applied on top of this content, respectively.
///
/// Only `Serialize` is needed here: this struct exists solely to be fed into
/// [`canonical_hash`], never to be parsed back out of JSON.
#[derive(Serialize)]
struct HashableRound<'a> {
    cigp_version: &'a str,
    round_id: &'a str,
    operator_id: &'a str,
    game_id: &'a str,
    game_version: &'a str,
    bet: &'a Money,
    payout: &'a Money,
    server_commitment: &'a str,
    client_seed: &'a str,
    nonce: u64,
    rng: &'a RngEvidence,
    mapping: &'a MappingEvidence,
    outcome: &'a serde_json::Value,
    paytable_hash: &'a str,
    game_logic_hash: &'a str,
    configuration_hash: &'a str,
    previous_round_hash: &'a str,
    timestamp: &'a str,
}

/// Compute the canonical `round_hash` for a proof, deliberately ignoring
/// whatever is currently in its `round_hash` and `signature` fields.
pub fn compute_round_hash(proof: &RoundProof) -> Result<String, CigpError> {
    let hashable = HashableRound {
        cigp_version: &proof.cigp_version,
        round_id: &proof.round_id,
        operator_id: &proof.operator_id,
        game_id: &proof.game_id,
        game_version: &proof.game_version,
        bet: &proof.bet,
        payout: &proof.payout,
        server_commitment: &proof.server_commitment,
        client_seed: &proof.client_seed,
        nonce: proof.nonce,
        rng: &proof.rng,
        mapping: &proof.mapping,
        outcome: &proof.outcome,
        paytable_hash: &proof.paytable_hash,
        game_logic_hash: &proof.game_logic_hash,
        configuration_hash: &proof.configuration_hash,
        previous_round_hash: &proof.previous_round_hash,
        timestamp: &proof.timestamp,
    };
    canonical_hash(&hashable)
}

/// Inputs required to build one round proof. RNG derivation and hashing are
/// performed internally; the caller supplies the revealed server seed,
/// client seed, nonce, and the already-hashed game-definition fingerprints.
pub struct RoundInputs {
    pub round_id: String,
    pub operator_id: String,
    pub game_id: String,
    pub game_version: String,
    pub bet: Money,
    pub payout: Money,
    pub server_seed: String,
    pub client_seed: String,
    pub nonce: u64,
    pub mapping_algorithm: String,
    pub mapping_version: String,
    pub mapping_parameters_hash: String,
    pub outcome: serde_json::Value,
    pub paytable_hash: String,
    pub game_logic_hash: String,
    pub configuration_hash: String,
    pub previous_round_hash: String,
    pub timestamp: String,
}

/// Build, hash, and sign a complete RoundProof from round inputs and an
/// operator keypair. The server seed is included in the returned proof
/// (post-reveal); operators publishing pre-reveal proofs should strip it.
pub fn build_round_proof(
    inputs: RoundInputs,
    keypair: &OperatorKeypair,
) -> Result<RoundProof, CigpError> {
    let server_commitment = commit(&inputs.server_seed);
    let rng_output = derive_rng_output(&inputs.server_seed, &inputs.client_seed, inputs.nonce);

    let mut proof = RoundProof {
        cigp_version: CIGP_VERSION.to_string(),
        round_id: inputs.round_id,
        operator_id: inputs.operator_id,
        game_id: inputs.game_id,
        game_version: inputs.game_version,
        bet: inputs.bet,
        payout: inputs.payout,
        server_commitment,
        server_seed: Some(inputs.server_seed),
        client_seed: inputs.client_seed,
        nonce: inputs.nonce,
        rng: RngEvidence {
            algorithm: cigp_core::types::REFERENCE_RNG_PROFILE.to_string(),
            version: "1".to_string(),
            output: rng_output,
        },
        mapping: MappingEvidence {
            algorithm: inputs.mapping_algorithm,
            version: inputs.mapping_version,
            parameters_hash: inputs.mapping_parameters_hash,
        },
        outcome: inputs.outcome,
        paytable_hash: inputs.paytable_hash,
        game_logic_hash: inputs.game_logic_hash,
        configuration_hash: inputs.configuration_hash,
        previous_round_hash: inputs.previous_round_hash,
        round_hash: String::new(),
        signature: String::new(),
        timestamp: inputs.timestamp,
    };

    let round_hash = compute_round_hash(&proof)?;
    proof.signature = keypair.sign(&round_hash);
    proof.round_hash = round_hash;

    Ok(proof)
}
