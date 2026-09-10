# LinkedIn Launch Text: CIGP Phase 1

**Author:** Ciprian Ștefan Pleșca

Today I am publishing the Phase 1 / Stage 1 research release of **CIGP: Casino Integrity & Gaming Proof Protocol**.

CIGP is an open-source research protocol for producing independently reproducible evidence about declared gaming-system events. The goal is deliberately narrow: make specific cryptographic claims inspectable instead of asking observers to rely on an operator's assertion.

Phase 1 implements a working reference path in Rust:

- deterministic `RoundProof` generation;
- SHA-256 commitments, HMAC-SHA-256 derivation, and Ed25519 signatures;
- a hash-linked audit ledger and Merkle inclusion proofs;
- an independent CLI verifier;
- a virtual-credit demo slot and a tamper-detection fixture.

The valid fixture reproduces as `VALID`; a payout-modified fixture produces `INVALID`. This is evidence infrastructure, not a casino platform, fairness certificate, regulatory approval, or real-money gambling product.

```mermaid
flowchart LR
    A[Declared round inputs] --> B[Signed RoundProof]
    B --> C[Independent reproduction]
    C --> D[VALID or INVALID]
```

The repository, academic wiki, manifesto, and reproducible test vectors are available at: https://github.com/Ciprian-LocalPulse/CIGP-Protocol

I welcome technical feedback on the cryptographic model, canonicalization, test vectors, threat model, and the boundaries between cryptographic verification, statistics, software integrity, and certification.

#opensource #rust #cryptography #securityengineering #reproducibility #gamingtechnology
