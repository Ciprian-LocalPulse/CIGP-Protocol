# Governance

## Purpose

CIGP-Protocol is intended to evolve as an open standard, comparable in spirit to how transport and security protocols such as TLS are maintained by open technical communities rather than a single commercial vendor. This document describes how decisions about the specification and reference implementation are currently made, during the pre-1.0 phase.

## Roles

- **Maintainer** — Ciprian Ștefan Pleșca, original author of the CIGP specification, holds final decision authority during the v0.x phase and merges changes to `spec/`, `schemas/`, and core crates.
- **Contributors** — anyone submitting issues, pull requests, test vectors, or specification proposals.
- **Reviewers** — contributors with demonstrated familiarity with a given subsystem (cryptography, statistics, verifier, compliance mapping) who are asked to review related pull requests.

## Decision process (pre-1.0)

1. Substantive changes to the specification (`spec/CIGP-0.1.md` and successors) are proposed as issues before implementation.
2. Discussion is expected to remain public and technical: proposals should reference concrete threat scenarios, test vectors, or regulatory requirements rather than general claims of "fairness" or "trust."
3. The maintainer merges changes once discussion has converged and required reviews (see `CONTRIBUTING.md`) are satisfied.

## Path to 1.0

As the project approaches `v1.0`, governance is expected to transition toward a steering group including independent contributors from cryptography, statistics, and gaming-compliance backgrounds, with formal versioning and a documented change-proposal process (similar in spirit to an RFC process). This section will be updated as that structure is established.

## Non-goals of governance

Governance of this repository does not extend to:

- Certifying that any specific operator or game is "fair."
- Acting as, or replacing, an accredited testing laboratory or gambling regulator.
- Making licensing decisions for any jurisdiction.
