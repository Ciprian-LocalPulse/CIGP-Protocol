//! Reference slot game using virtual credits only.
//! This crate intentionally has no wallet, network, or real-money behavior.

use cigp_core::{Money, RoundProof};
use cigp_crypto::OperatorKeypair;
use cigp_proof::{build_round_proof, RoundInputs};
use serde_json::json;

const SERVER_SEED: &str = "22";
const PAYTABLE_HASH: &str =
    "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
const GAME_LOGIC_HASH: &str =
    "sha256:bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";
const CONFIGURATION_HASH: &str =
    "sha256:cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc";

pub fn symbols_from_rng(rng_hex: &str) -> [String; 3] {
    let bytes = hex::decode(rng_hex).unwrap_or_default();
    let names = ["CHERRY", "LEMON", "BAR", "SEVEN"];
    [
        names[bytes.first().copied().unwrap_or_default() as usize % names.len()].into(),
        names[bytes.get(1).copied().unwrap_or_default() as usize % names.len()].into(),
        names[bytes.get(2).copied().unwrap_or_default() as usize % names.len()].into(),
    ]
}

pub fn payout_minor_units(symbols: &[String; 3], bet_minor_units: i64) -> i64 {
    if symbols[0] == symbols[1] && symbols[1] == symbols[2] {
        bet_minor_units * if symbols[0] == "SEVEN" { 50 } else { 10 }
    } else if symbols[0] == symbols[1] || symbols[1] == symbols[2] || symbols[0] == symbols[2] {
        bet_minor_units * 2
    } else {
        0
    }
}

pub fn build_demo_round(keypair: &OperatorKeypair) -> Result<RoundProof, cigp_core::CigpError> {
    build_demo_round_with(keypair, 0, "sha256:genesis")
}

/// Construct one deterministic virtual-credit round in a hash-linked sequence.
/// This exists solely for research fixtures and must not be used with a fixed
/// seed in an operational deployment.
pub fn build_demo_round_with(
    keypair: &OperatorKeypair,
    nonce: u64,
    previous_round_hash: &str,
) -> Result<RoundProof, cigp_core::CigpError> {
    let server_seed = SERVER_SEED.repeat(32);
    let client_seed = format!("cigp-demo-client-{nonce}");
    let rng = cigp_crypto::derive_rng_output(&server_seed, &client_seed, nonce);
    let symbols = symbols_from_rng(&rng);
    let bet =
        Money::new("EUR", 100).map_err(|e| cigp_core::CigpError::InvalidField(e.to_string()))?;
    let payout = Money::new("EUR", payout_minor_units(&symbols, bet.minor_units))
        .map_err(|e| cigp_core::CigpError::InvalidField(e.to_string()))?;
    build_round_proof(
        RoundInputs {
            round_id: format!("demo-round-{nonce:08}"),
            operator_id: "cigp-reference-operator".into(),
            game_id: "cigp-demo-slot".into(),
            game_version: "1.0.0".into(),
            bet,
            payout,
            server_seed,
            client_seed,
            nonce,
            mapping_algorithm: "CIGP-DEMO-SLOT-MODULO".into(),
            mapping_version: "1".into(),
            mapping_parameters_hash: PAYTABLE_HASH.into(),
            outcome: json!({"symbols": symbols}),
            paytable_hash: PAYTABLE_HASH.into(),
            game_logic_hash: GAME_LOGIC_HASH.into(),
            configuration_hash: CONFIGURATION_HASH.into(),
            previous_round_hash: previous_round_hash.into(),
            timestamp: format!("2026-01-01T00:00:{:02}Z", nonce % 60),
        },
        keypair,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use cigp_crypto::OperatorKeypair;
    use cigp_proof::verify_round_proof;

    #[test]
    fn demo_round_is_virtual_and_verifiable() {
        let keypair = OperatorKeypair::from_seed_hex(&"11".repeat(32)).unwrap();
        let proof = build_demo_round(&keypair).unwrap();
        assert!(verify_round_proof(&proof, &keypair.public_key_hex())
            .unwrap()
            .is_valid());
        assert_eq!(proof.bet.currency_code(), "EUR");
    }

    #[test]
    fn linked_rounds_refer_to_the_previous_round_hash() {
        let keypair = OperatorKeypair::from_seed_hex(&"11".repeat(32)).unwrap();
        let first = build_demo_round_with(&keypair, 0, "sha256:genesis").unwrap();
        let second = build_demo_round_with(&keypair, 1, &first.round_hash).unwrap();
        assert_eq!(second.previous_round_hash, first.round_hash);
        assert!(verify_round_proof(&second, &keypair.public_key_hex())
            .unwrap()
            .is_valid());
    }
}
