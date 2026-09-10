# Architecture

**Author:** Ciprian Ștefan Pleșca

The Phase 1 architecture separates domain representation, cryptography, proof construction, ledger integrity, and verification. This separation makes it possible to inspect each responsibility independently.

```mermaid
flowchart TB
    CORE[cigp-core\nTyped domain model and canonicalization]
    CRYPTO[cigp-crypto\nSHA-256, HMAC, Ed25519, Merkle]
    PROOF[cigp-proof\nRoundProof construction and verification]
    LEDGER[cigp-ledger\nHash-linked append-only ledger]
    CLI[cigp CLI\nGeneration and independent verification]
    SIM[cigp-demo-slot\nVirtual-credit reference simulator]

    CORE --> PROOF
    CRYPTO --> PROOF
    PROOF --> LEDGER
    SIM --> PROOF
    PROOF --> CLI
    LEDGER --> CLI
```

## Evidence Lifecycle

```mermaid
sequenceDiagram
    participant S as Reference simulator
    participant P as Proof engine
    participant L as Audit ledger
    participant V as Verifier

    S->>P: deterministic inputs and virtual outcome
    P->>P: commitment, RNG evidence, round hash, signature
    P->>L: append hash-linked proof
    L->>L: construct Merkle batch
    P-->>V: proof plus public key
    V->>V: recompute cryptographic checks
```

The data boundary is intentional: the proof contains evidence required for verification, not player identity, payment information, passwords, or production secrets.

