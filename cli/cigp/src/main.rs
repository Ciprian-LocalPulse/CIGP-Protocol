use cigp_core::RoundProof;
use cigp_crypto::OperatorKeypair;
use cigp_demo_slot::build_demo_round;
use cigp_proof::verify_round_proof;
use std::{env, fs, path::PathBuf, process::ExitCode};

const DEFAULT_SEED: &str = "11";

fn usage() {
    eprintln!("Usage: cigp generate-demo-round [directory]\n       cigp verify <proof.json> [public-key-hex]");
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
    let key_path = directory.join("demo-round.pubkey");
    write_json(&proof_path, &proof)?;
    fs::write(&key_path, format!("{}\n", keypair.public_key_hex()))
        .map_err(|e| format!("{}: {e}", key_path.display()))?;

    let mut tampered = proof.clone();
    tampered.payout.minor_units += 1;
    write_json(&directory.join("tampered-round.json"), &tampered)?;
    fs::write(
        directory.join("tampered-round.pubkey"),
        format!("{}\n", keypair.public_key_hex()),
    )
    .map_err(|e| format!("tampered public key: {e}"))?;
    println!("Generated {}", proof_path.display());
    println!("Public key: {}", key_path.display());
    println!(
        "Generated tamper fixture: {}",
        directory.join("tampered-round.json").display()
    );
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

fn main() -> ExitCode {
    let mut args = env::args().skip(1);
    let result = match args.next().as_deref() {
        Some("generate-demo-round") => {
            generate(PathBuf::from(args.next().unwrap_or_else(|| ".".into()))).map(|_| true)
        }
        Some("verify") => match args.next() {
            Some(path) => verify(PathBuf::from(path), args.next()).map_err(|e| e),
            None => {
                usage();
                Err("missing proof path".into())
            }
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
