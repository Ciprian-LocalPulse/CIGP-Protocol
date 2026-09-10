//! End-to-end demonstration: build a valid RoundProof, verify it, then
//! deliberately tamper with the payout field and show that independent
//! verification detects it.
//!
//! Run with: `cargo run -p cigp-proof --example tamper_demo`

use cigp_core::Money;
use cigp_crypto::OperatorKeypair;
use cigp_proof::{build_round_proof, verify_round_proof, RoundInputs};
use serde_json::json;

fn sample_inputs() -> RoundInputs {
    RoundInputs {
        round_id: "round-demo-0001".into(),
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

fn main() {
    let kp = OperatorKeypair::generate();

    // 1. Build a valid, signed round proof.
    let proof = build_round_proof(sample_inputs(), &kp).expect("build_round_proof");
    let report = verify_round_proof(&proof, &kp.public_key_hex()).expect("verify_round_proof");

    println!("CIGP Verification");
    println!("Protocol version: {}", report.protocol_version);
    println!("Round ID: {}", report.round_id);
    println!("Signature: {}", report.signature);
    println!("Commitment: {}", report.commitment);
    println!("Seed derivation: {}", report.seed_derivation);
    println!("Round hash: {}", report.round_hash);
    println!(
        "RESULT: {}",
        if report.is_valid() {
            "VALID"
        } else {
            "INVALID"
        }
    );

    // 2. Tamper with the payout after the fact (e.g. a compromised database
    //    row) and show that verification now fails.
    let mut tampered = proof.clone();
    tampered.payout = Money::new("EUR", 999_999).unwrap();
    let tampered_report =
        verify_round_proof(&tampered, &kp.public_key_hex()).expect("verify_round_proof");

    println!();
    println!("CIGP Verification (tampered payout)");
    println!("Round ID: {}", tampered_report.round_id);
    println!("Signature: {}", tampered_report.signature);
    println!("Round hash: {}", tampered_report.round_hash);
    println!(
        "RESULT: {}",
        if tampered_report.is_valid() {
            "VALID"
        } else {
            "INVALID"
        }
    );

    assert!(report.is_valid());
    assert!(!tampered_report.is_valid());
}
