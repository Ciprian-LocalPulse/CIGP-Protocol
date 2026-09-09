# Contributing to CIGP-Protocol

Thank you for your interest in the **Casino Integrity & Gaming Proof Protocol (CIGP)**. This project is an open specification and reference implementation, and it depends on outside review to be credible — please read this before opening an issue or a pull request.

## Ways to contribute

- **Specification review** — propose clarifications, edge cases, or corrections to `spec/CIGP-0.1.md` and the schemas under `schemas/`.
- **Reference implementation** — Rust (crypto core, proof engine, ledger, CLI), Julia (statistics engine), Python (anomaly detection), TypeScript (verifier, SDK).
- **Test vectors** — additional known-input/known-output pairs under `test-vectors/`, including adversarial and edge cases.
- **Threat modelling** — extensions to `docs/threat-model.md`.
- **Compliance mapping** — mapping CIGP objects to specific jurisdictional requirements under `compliance/`.

## Ground rules

1. **No unverifiable claims.** Any change that adds a new "verified" or "certified" claim must include the mechanism by which a third party can independently check it.
2. **Standard cryptography only.** Do not introduce custom or unreviewed cryptographic primitives. Proposals to add a primitive must reference an established, peer-reviewed standard.
3. **Backwards-compatible schema changes preferred.** Breaking changes to `schemas/*.schema.json` require a version bump and a migration note in `spec/`.
4. **Every claim needs a test vector.** Changes to proof logic should be accompanied by at least one test vector under `test-vectors/`.

## Pull request process

1. Open an issue describing the problem or proposal before submitting a large PR.
2. Reference the relevant section of `spec/CIGP-0.1.md` in your PR description.
3. Include tests (unit tests for code changes, test vectors for protocol changes).
4. One reviewer approval is required for documentation changes; two are required for changes to `crates/cigp-crypto` or `crates/cigp-proof`.

## Reporting security issues

Do **not** open a public issue for a suspected vulnerability. See [`SECURITY.md`](SECURITY.md).

## Code of conduct

All contributors are expected to follow the [Code of Conduct](CODE_OF_CONDUCT.md).
