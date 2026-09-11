use cigp_core::RoundProof;
use cigp_crypto::OperatorKeypair;
use cigp_demo_slot::{build_demo_round, build_demo_round_with};
use cigp_ledger::JsonlLedger;
use cigp_proof::verify_round_proof;
use cigp_statistics::rtp_report;
use std::{env, fs, path::PathBuf, process::ExitCode};

const DEFAULT_SEED: &str = "11";

fn usage() {
    eprintln!("Usage:\n  cigp generate-demo-round [directory]\n  cigp generate-demo-ledger <round-count> [directory]\n  cigp verify <proof.json> [public-key-hex]\n  cigp ledger-verify <ledger.jsonl> <public-key-hex>\n  cigp stats <ledger.jsonl>");
}

fn write_public_key(
    directory: &PathBuf,
    name: &str,
    keypair: &OperatorKeypair,
) -> Result<(), String> {
    let key_path = directory.join(name);
    fs::write(&key_path, format!("{}\n", keypair.public_key_hex()))
        .map_err(|e| format!("{}: {e}", key_path.display()))
}

fn write_json(path: &PathBuf, value: &impl serde::Serialize) -> Result<(), String> {
    let file = fs::File::create(path).map_err(|e| format!("{}: {e}", path.display()))?;
    serde_json::to_writer_pretty(file, value).map_err(|e| format!("{}: {e}", path.display()))
}

fn generate(directory: PathBuf) -> Result<(), String> {
    fs::create_dir_all(&directory).map_err(|e| format!("{}: {e}", directory.display()))?;
    let keypair = OperatorKeypair::from_seed_hex(&DEFAULT_SEED.repeat(32))
        .map_err(|e| format!("keypair: {e}"))?;
    let proof = build_demo_round(&keypair).map_err(|e| format!("build proof: {e}"))?;
    let proof_path = directory.join("demo-round.json");
    write_json(&proof_path, &proof)?;
    write_public_key(&directory, "demo-round.pubkey", &keypair)?;

    let mut tampered = proof.clone();
    tampered.payout.minor_units += 1;
    write_json(&directory.join("tampered-round.json"), &tampered)?;
    write_public_key(&directory, "tampered-round.pubkey", &keypair)?;
    println!("Generated {}", proof_path.display());
    println!(
        "Public key: {}",
        directory.join("demo-round.pubkey").display()
    );
    println!(
        "Generated tamper fixture: {}",
        directory.join("tampered-round.json").display()
    );
    Ok(())
}

fn generate_ledger(round_count: usize, directory: PathBuf) -> Result<(), String> {
    if round_count == 0 {
        return Err("round-count must be greater than zero".into());
    }
    fs::create_dir_all(&directory).map_err(|e| format!("{}: {e}", directory.display()))?;
    let keypair = OperatorKeypair::from_seed_hex(&DEFAULT_SEED.repeat(32))
        .map_err(|e| format!("keypair: {e}"))?;
    let ledger_path = directory.join("demo-ledger.jsonl");
    if ledger_path.exists() {
        return Err(format!(
            "refusing to overwrite existing ledger: {}",
            ledger_path.display()
        ));
    }
    let mut ledger = JsonlLedger::open(&ledger_path).map_err(|e| e.to_string())?;
    let mut previous = "sha256:genesis".to_string();
    for nonce in 0..round_count as u64 {
        let proof = build_demo_round_with(&keypair, nonce, &previous)
            .map_err(|e| format!("build round {nonce}: {e}"))?;
        previous = proof.round_hash.clone();
        ledger.append(proof).map_err(|e| e.to_string())?;
    }
    let bets: Vec<i64> = ledger
        .ledger()
        .rounds()
        .iter()
        .map(|proof| proof.bet.minor_units)
        .collect();
    let payouts: Vec<i64> = ledger
        .ledger()
        .rounds()
        .iter()
        .map(|proof| proof.payout.minor_units)
        .collect();
    let report = rtp_report(&bets, &payouts).map_err(|e| e.to_string())?;
    write_json(&directory.join("demo-statistics.json"), &report)?;
    write_public_key(&directory, "demo-ledger.pubkey", &keypair)?;
    println!(
        "Generated {} rounds in {}",
        round_count,
        ledger_path.display()
    );
    println!(
        "Merkle root: {}",
        ledger
            .ledger()
            .merkle_batch()
            .map_err(|e| e.to_string())?
            .root_hex()
    );
    println!("Observed RTP: {:.6}", report.observed_rtp);
    Ok(())
}

fn verify(path: PathBuf, key_arg: Option<String>) -> Result<bool, String> {
    let bytes = fs::read(&path).map_err(|e| format!("{}: {e}", path.display()))?;
    let proof: RoundProof =
        serde_json::from_slice(&bytes).map_err(|e| format!("malformed proof: {e}"))?;
    let public_key = match key_arg {
        Some(key) => key,
        None => {
            let sidecar = path.with_extension("pubkey");
            fs::read_to_string(&sidecar)
                .map_err(|e| {
                    format!(
                        "public key missing; pass it as the second argument ({}) : {e}",
                        sidecar.display()
                    )
                })?
                .trim()
                .to_string()
        }
    };
    let report = verify_round_proof(&proof, &public_key).map_err(|e| e.to_string())?;
    println!("CIGP Verification");
    println!("Protocol version: {}", report.protocol_version);
    println!("Round ID:         {}", report.round_id);
    println!("Signature:        {}", report.signature);
    println!("Commitment:       {}", report.commitment);
    println!("Seed derivation:  {}", report.seed_derivation);
    println!("Round hash:       {}", report.round_hash);
    println!(
        "RESULT: {}",
        if report.is_valid() {
            "VALID"
        } else {
            "INVALID"
        }
    );
    Ok(report.is_valid())
}

fn verify_ledger(path: PathBuf, public_key: String) -> Result<bool, String> {
    let ledger = JsonlLedger::open(&path).map_err(|e| e.to_string())?;
    if ledger.ledger().is_empty() {
        return Err("ledger is empty".into());
    }
    ledger.ledger().verify_chain().map_err(|e| e.to_string())?;
    let invalid = ledger.ledger().rounds().iter().find(|proof| {
        verify_round_proof(proof, &public_key).map_or(true, |report| !report.is_valid())
    });
    println!("CIGP Ledger Verification");
    println!("Rounds: {}", ledger.ledger().len());
    println!("Hash chain: PASS");
    println!(
        "Merkle root: {}",
        ledger
            .ledger()
            .merkle_batch()
            .map_err(|e| e.to_string())?
            .root_hex()
    );
    if let Some(proof) = invalid {
        println!("Invalid round: {}", proof.round_id);
        println!("RESULT: INVALID");
        Ok(false)
    } else {
        println!("Proof checks: PASS");
        println!("RESULT: VALID");
        Ok(true)
    }
}

fn statistics(path: PathBuf) -> Result<(), String> {
    let ledger = JsonlLedger::open(path).map_err(|e| e.to_string())?;
    let bets: Vec<i64> = ledger
        .ledger()
        .rounds()
        .iter()
        .map(|proof| proof.bet.minor_units)
        .collect();
    let payouts: Vec<i64> = ledger
        .ledger()
        .rounds()
        .iter()
        .map(|proof| proof.payout.minor_units)
        .collect();
    let report = rtp_report(&bets, &payouts).map_err(|e| e.to_string())?;
    println!("CIGP Descriptive Statistics");
    println!("Rounds: {}", report.sample_size);
    println!("Observed RTP: {:.6}", report.observed_rtp);
    println!("Payout variance: {:.6}", report.payout_variance);
    println!(
        "95% interval: [{:.6}, {:.6}]",
        report.confidence_interval_95[0], report.confidence_interval_95[1]
    );
    println!("Interpretation: DESCRIPTIVE ONLY - not proof of fairness");
    Ok(())
}

fn main() -> ExitCode {
    let mut args = env::args().skip(1);
    let result = match args.next().as_deref() {
        Some("generate-demo-round") => {
            generate(PathBuf::from(args.next().unwrap_or_else(|| ".".into()))).map(|_| true)
        }
        Some("generate-demo-ledger") => match args.next() {
            Some(count) => match count.parse::<usize>() {
                Ok(count) => generate_ledger(
                    count,
                    PathBuf::from(args.next().unwrap_or_else(|| ".".into())),
                )
                .map(|_| true),
                Err(_) => Err("round-count must be an unsigned integer".into()),
            },
            None => Err("missing round-count".into()),
        },
        Some("verify") => match args.next() {
            Some(path) => verify(PathBuf::from(path), args.next()).map_err(|e| e),
            None => {
                usage();
                Err("missing proof path".into())
            }
        },
        Some("ledger-verify") => match (args.next(), args.next()) {
            (Some(path), Some(public_key)) => verify_ledger(PathBuf::from(path), public_key),
            _ => Err("ledger-verify requires <ledger.jsonl> <public-key-hex>".into()),
        },
        Some("stats") => match args.next() {
            Some(path) => statistics(PathBuf::from(path)).map(|_| true),
            None => Err("stats requires <ledger.jsonl>".into()),
        },
        _ => {
            usage();
            Err("invalid command".into())
        }
    };
    match result {
        Ok(true) => ExitCode::SUCCESS,
        Ok(false) => ExitCode::from(1),
        Err(error) => {
            eprintln!("error: {error}");
            ExitCode::from(2)
        }
    }
}
