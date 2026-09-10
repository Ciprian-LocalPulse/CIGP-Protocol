# Audit Ledger

The reference ledger is an append-only ordered collection of `RoundProof` values. Each event carries the previous event hash, beginning at `sha256:genesis`. `AuditLedger::verify_chain` recomputes link relationships independently of storage.

The current implementation is in-memory and storage-backend agnostic. A durable database adapter is intentionally outside the v0.1 core.
