# CIGP Threat Model

This document describes the evidence boundary. CIGP detects inconsistencies in supplied evidence; it does not attest that an entire operator environment is benign.

| Asset | Attacker capability | Attack | Mitigation | Residual risk |
| --- | --- | --- | --- | --- |
| Server seed commitment | Malicious operator | Change seed after play | SHA-256 commitment checked after reveal | A seed can be withheld or disclosed late |
| Round proof | Insider or compromised server | Edit payout, outcome, nonce, or hashes | Canonical round hash and Ed25519 signature | Compromised signing key |
| Hash chain | Storage attacker | Reorder or delete events | Previous-hash verification | Missing external checkpoint |
| Merkle batch | Batch tampering | Replace a leaf or path | Domain-separated inclusion proof | Unpublished or untrusted root |
| Game definition | Provider or update attacker | Change logic or paytable | Published fingerprint hashes | A verifier may use the wrong manifest |
| Client seed | Malicious client | Replay or choose biased input | Unique nonce policy and audit trail | Client-side intent is outside CIGP |
| Dependencies | Supply-chain attacker | Publish compromised library | Lockfile, CI audit, review policy | Zero-day vulnerabilities |

Physical machine attestation is design-only in v0.1. CIGP does not control real machines, wallets, payments, or casino operations.
