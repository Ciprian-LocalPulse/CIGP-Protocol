# Limitations and Roadmap

**Author:** Ciprian Ștefan Pleșca

## Current Limitations

- The Rust verifier checks internal cryptographic consistency, but the full independent browser verifier remains research work.
- Julia, Python, and TypeScript components are reference utilities rather than production services.
- The in-memory audit ledger is storage-backend agnostic; durable operational storage is not implemented.
- Statistical consistency, software attestation, and certification are distinct workstreams.
- Physical-machine integration is design-only.

```mermaid
flowchart LR
    P1[Phase 1\nReproducible evidence loop] --> P2[Phase 2\nStatistical methods and cross-language vectors]
    P2 --> P3[Phase 3\nDurable storage, APIs, operational controls]
    P3 --> P4[Phase 4\nIndependent review and deployment research]
```

## Research Discipline

Each phase must preserve the distinction below.

```mermaid
flowchart TB
    C[Cryptographic evidence] --> A[Can supplied data be reproduced?]
    S[Statistical evidence] --> B[Is a sample consistent with a declared model?]
    T[Technical attestation] --> D[Does a measured artifact match a declared artifact?]
    R[Regulatory evidence] --> E[Has an authorized body made a decision?]
```

These categories are complementary; none should be substituted for another in public claims.

