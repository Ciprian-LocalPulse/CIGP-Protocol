//! `cigp-ledger` — append-only, hash-linked audit ledger over round proofs,
//! with Merkle batch commitments (CIGP §14–15).
//!
//! The ledger does not trust its own storage backend: chain verification
//! recomputes every link from the stored `RoundProof` content, so tampering
//! with any stored round — including reordering, deleting, or editing
//! entries — is independently detectable without relying on database
//! guarantees.

use cigp_core::RoundProof;
use cigp_crypto::{MerkleError, MerkleTree};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use thiserror::Error;

pub const GENESIS_HASH: &str = "sha256:genesis";

#[derive(Debug, Error)]
pub enum LedgerError {
    #[error("round hash chain broken at round_id={round_id}: expected previous_round_hash={expected}, found {actual}")]
    ChainBroken {
        round_id: String,
        expected: String,
        actual: String,
    },
    #[error("ledger is empty")]
    Empty,
    #[error(transparent)]
    Merkle(#[from] MerkleError),
    #[error("ledger I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("malformed JSONL ledger entry at line {line}: {source}")]
    MalformedEntry {
        line: usize,
        #[source]
        source: serde_json::Error,
    },
}

/// An in-memory (or backing-store-agnostic) append-only ledger of round
/// proofs, ordered by insertion.
#[derive(Default)]
pub struct AuditLedger {
    rounds: Vec<RoundProof>,
}

/// A durable JSON Lines adapter around [`AuditLedger`]. Each line contains
/// one serialized `RoundProof`; readers rebuild the logical chain instead of
/// trusting the storage implementation or its metadata.
pub struct JsonlLedger {
    path: PathBuf,
    ledger: AuditLedger,
}

impl JsonlLedger {
    /// Open an existing ledger or initialise an empty logical ledger at `path`.
    pub fn open(path: impl AsRef<Path>) -> Result<Self, LedgerError> {
        let path = path.as_ref().to_path_buf();
        let mut ledger = AuditLedger::new();
        if path.exists() {
            let file = File::open(&path)?;
            for (index, line) in BufReader::new(file).lines().enumerate() {
                let line = line?;
                if line.trim().is_empty() {
                    continue;
                }
                let proof =
                    serde_json::from_str(&line).map_err(|source| LedgerError::MalformedEntry {
                        line: index + 1,
                        source,
                    })?;
                ledger.append(proof)?;
            }
        }
        Ok(Self { path, ledger })
    }

    /// Validate the next proof, then synchronously flush it before advancing
    /// the in-memory chain. A failed file write cannot create a memory-only
    /// event that appears persisted.
    pub fn append(&mut self, proof: RoundProof) -> Result<(), LedgerError> {
        self.ledger.validate_next(&proof)?;
        let encoded = serde_json::to_string(&proof)
            .expect("RoundProof serialization is infallible for JSON values");
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)?;
        writeln!(file, "{encoded}")?;
        file.sync_data()?;
        self.ledger.append(proof)?;
        Ok(())
    }

    pub fn ledger(&self) -> &AuditLedger {
        &self.ledger
    }
}

impl AuditLedger {
    pub fn new() -> Self {
        AuditLedger { rounds: Vec::new() }
    }

    /// Append a round proof. Does not itself validate the proof's
    /// cryptographic content — use `cigp-proof::verify_round_proof` for
    /// that. This only enforces that the chain link is structurally
    /// consistent with the current ledger tip.
    pub fn append(&mut self, proof: RoundProof) -> Result<(), LedgerError> {
        self.validate_next(&proof)?;
        self.rounds.push(proof);
        Ok(())
    }

    fn validate_next(&self, proof: &RoundProof) -> Result<(), LedgerError> {
        let expected_previous = self
            .rounds
            .last()
            .map(|r| r.round_hash.clone())
            .unwrap_or_else(|| GENESIS_HASH.to_string());

        if proof.previous_round_hash != expected_previous {
            Err(LedgerError::ChainBroken {
                round_id: proof.round_id.clone(),
                expected: expected_previous,
                actual: proof.previous_round_hash.clone(),
            })
        } else {
            Ok(())
        }
    }

    pub fn len(&self) -> usize {
        self.rounds.len()
    }

    pub fn is_empty(&self) -> bool {
        self.rounds.is_empty()
    }

    pub fn rounds(&self) -> &[RoundProof] {
        &self.rounds
    }

    /// Verify that every stored round's `previous_round_hash` correctly
    /// links to the round before it, forming an unbroken chain from
    /// genesis. Does not re-verify individual round signatures/hashes —
    /// combine with `cigp-proof::verify_round_proof` for full assurance.
    pub fn verify_chain(&self) -> Result<(), LedgerError> {
        let mut expected_previous = GENESIS_HASH.to_string();
        for round in &self.rounds {
            if round.previous_round_hash != expected_previous {
                return Err(LedgerError::ChainBroken {
                    round_id: round.round_id.clone(),
                    expected: expected_previous,
                    actual: round.previous_round_hash.clone(),
                });
            }
            expected_previous = round.round_hash.clone();
        }
        Ok(())
    }

    /// Build a Merkle tree over the current ledger's round hashes, for
    /// batch anchoring / inclusion-proof issuance.
    pub fn merkle_batch(&self) -> Result<MerkleTree, LedgerError> {
        if self.rounds.is_empty() {
            return Err(LedgerError::Empty);
        }
        let leaves: Vec<Vec<u8>> = self
            .rounds
            .iter()
            .map(|r| r.round_hash.clone().into_bytes())
            .collect();
        Ok(MerkleTree::build(&leaves)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cigp_core::Money;

    fn dummy_round(id: &str, previous: &str, hash: &str) -> RoundProof {
        RoundProof {
            cigp_version: "0.1".into(),
            round_id: id.into(),
            operator_id: "op".into(),
            game_id: "game".into(),
            game_version: "1.0.0".into(),
            bet: Money::new("EUR", 100).unwrap(),
            payout: Money::new("EUR", 0).unwrap(),
            server_commitment: "commitment".into(),
            server_seed: None,
            client_seed: "seed".into(),
            nonce: 0,
            rng: cigp_core::types::RngEvidence {
                algorithm: "test".into(),
                version: "1".into(),
                output: "output".into(),
            },
            mapping: cigp_core::types::MappingEvidence {
                algorithm: "test".into(),
                version: "1".into(),
                parameters_hash: "hash".into(),
            },
            outcome: serde_json::json!({}),
            paytable_hash: "hash".into(),
            game_logic_hash: "hash".into(),
            configuration_hash: "hash".into(),
            previous_round_hash: previous.into(),
            round_hash: hash.into(),
            signature: "sig".into(),
            timestamp: "2026-01-01T00:00:00Z".into(),
        }
    }

    #[test]
    fn accepts_correctly_linked_chain() {
        let mut ledger = AuditLedger::new();
        ledger
            .append(dummy_round("r1", GENESIS_HASH, "hash1"))
            .unwrap();
        ledger.append(dummy_round("r2", "hash1", "hash2")).unwrap();
        assert!(ledger.verify_chain().is_ok());
    }

    #[test]
    fn rejects_broken_link_on_append() {
        let mut ledger = AuditLedger::new();
        ledger
            .append(dummy_round("r1", GENESIS_HASH, "hash1"))
            .unwrap();
        let err = ledger.append(dummy_round("r2", "wrong-previous", "hash2"));
        assert!(err.is_err());
    }

    #[test]
    fn detects_post_hoc_tampering_via_verify_chain() {
        let mut ledger = AuditLedger::new();
        ledger
            .append(dummy_round("r1", GENESIS_HASH, "hash1"))
            .unwrap();
        ledger.append(dummy_round("r2", "hash1", "hash2")).unwrap();
        // Simulate storage-level tampering: edit round 1's hash directly.
        ledger.rounds[0].round_hash = "tampered".into();
        assert!(ledger.verify_chain().is_err());
    }

    #[test]
    fn merkle_batch_and_inclusion_proof() {
        let mut ledger = AuditLedger::new();
        ledger
            .append(dummy_round("r1", GENESIS_HASH, "hash1"))
            .unwrap();
        ledger.append(dummy_round("r2", "hash1", "hash2")).unwrap();
        ledger.append(dummy_round("r3", "hash2", "hash3")).unwrap();
        let tree = ledger.merkle_batch().unwrap();
        for (i, round) in ledger.rounds().iter().enumerate() {
            let proof = tree.prove(i).unwrap();
            assert!(cigp_crypto::merkle::verify_inclusion(
                round.round_hash.as_bytes(),
                &proof,
                &tree.root()
            ));
        }
    }

    #[test]
    fn jsonl_ledger_persists_and_reloads_chain() {
        let path = std::env::temp_dir().join(format!("cigp-ledger-{}.jsonl", std::process::id()));
        let _ = std::fs::remove_file(&path);
        let mut durable = JsonlLedger::open(&path).unwrap();
        durable
            .append(dummy_round("r1", GENESIS_HASH, "hash1"))
            .unwrap();
        durable.append(dummy_round("r2", "hash1", "hash2")).unwrap();
        drop(durable);

        let reloaded = JsonlLedger::open(&path).unwrap();
        assert_eq!(reloaded.ledger().len(), 2);
        assert!(reloaded.ledger().verify_chain().is_ok());
        std::fs::remove_file(path).unwrap();
    }
}
