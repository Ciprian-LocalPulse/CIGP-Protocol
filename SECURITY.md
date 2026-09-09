# Security Policy

## Scope

CIGP-Protocol is evidence and verification infrastructure for gaming integrity. Vulnerabilities in this repository are taken seriously because they could undermine the credibility of proofs relied upon by players, operators, laboratories, or regulators.

Security-relevant components include, but are not limited to:

- `crates/cigp-crypto` — hashing, signing, canonicalization
- `crates/cigp-proof` — RoundProof generation and verification
- `crates/cigp-ledger` — Merkle audit ledger
- `crates/cigp-verifier` — independent verifier logic
- `schemas/` — normative JSON schemas
- `julia/CIGPStatistics.jl` — statistical testing engine

## Reporting a Vulnerability

**Please do not open a public GitHub issue for security vulnerabilities.**

Instead, report privately via GitHub's private vulnerability reporting feature on this repository, or contact the maintainer directly through the contact details listed on the maintainer's GitHub profile (`github.com/Ciprian-LocalPulse`).

Please include:

- A description of the vulnerability and its potential impact on proof validity or verifier correctness.
- Steps to reproduce, including a minimal test vector if applicable.
- Any suggested remediation.

## Response Process

1. Acknowledgement of the report.
2. Confirmation and severity assessment.
3. Coordinated disclosure timeline agreed with the reporter.
4. Patch release accompanied by a security advisory and, where relevant, an updated test vector demonstrating the fix.

## Out of Scope

- Vulnerabilities in third-party casino platforms that *implement* CIGP are not in scope for this repository; report those to the relevant operator.
- CIGP does not certify game fairness or replace accredited testing laboratories; issues concerning licensing or certification decisions of any jurisdiction are out of scope.
