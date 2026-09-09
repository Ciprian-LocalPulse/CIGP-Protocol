# CIGP Specification — Version 0.1 (Draft)

**Title:** Casino Integrity & Gaming Proof Protocol
**Status:** Draft — open for public review
**Author:** Ciprian Ștefan Pleșca

## 0. Notational Conventions

The key words "MUST", "MUST NOT", "SHOULD", "SHOULD NOT", and "MAY" in this document are to be interpreted as commonly understood in protocol specifications (cf. RFC 2119 conventions), even though this document is not itself an IETF RFC.

## 1. Scope

This document specifies:

1. The canonical serialization format used for all CIGP objects.
2. The `RoundProof` object and its validation procedure.
3. The seed commitment scheme.
4. Game logic and configuration fingerprinting.
5. The Merkle-based audit ledger and anchoring procedure.
6. The four CIGP trust levels and the claims each one supports.

This document does **not** specify a particular RNG hardware implementation, a specific jurisdiction's certification process, or a betting/payments system. Those are treated as external, complementary concerns.

## 2. Canonical Serialization

All CIGP objects that are hashed or signed MUST first be serialized using **JSON Canonicalization Scheme (JCS, RFC 8785)** or an equivalent deterministic canonical form, so that independent implementations compute identical hashes from identical logical content.

## 3. Cryptographic Primitives (v0.1)

| Purpose | Primitive |
|---|---|
| Hashing | SHA-256 |
| Message authentication / derivation | HMAC-SHA-256 |
| Key derivation | HKDF |
| Digital signatures | Ed25519 |
| Structural commitments | Merkle Trees (binary, SHA-256 leaves) |

Implementations MUST NOT substitute non-standard or unreviewed primitives for those listed above in v0.1.

## 4. Seed Commitment Scheme

1. The operator generates a secret `server_seed` prior to a round (or a batch of rounds).
2. The operator publishes `server_commitment = SHA256(server_seed)` **before** the round is played.
3. The player, if the game supports it, supplies a `client_seed`.
4. Each round carries a `nonce`, which MUST be unique per `server_seed`.
5. The round's RNG-derived value is computed as `HMAC-SHA256(server_seed, client_seed ‖ nonce)`.
6. At a later point (end of session, or on a rolling basis), the operator reveals `server_seed`.
7. Any verifier MUST be able to confirm `SHA256(revealed_server_seed) == server_commitment` published prior to play.

This scheme binds the operator to a seed before the outcome-relevant randomness is consumed, while allowing external verification after reveal.

## 5. RoundProof Object

The `RoundProof` is the atomic unit of evidence in CIGP. Its normative schema is defined in [`schemas/round-proof.schema.json`](../schemas/round-proof.schema.json). A `RoundProof` MUST include:

- `round_id`, `operator_id`, `game_id`, `game_version`
- `bet`, `currency`
- `server_commitment`, `client_seed`, `nonce`
- `rng` (algorithm, version, output)
- `mapping` (algorithm, version, parameters hash)
- `outcome`
- `paytable_hash`, `game_logic_hash`
- `payout`
- `previous_round_hash`, `round_hash`
- `signature`
- `timestamp` (ISO 8601, UTC)

### 5.1 Round Hash Chain

`round_hash` for round *n* MUST be computed over the canonical serialization of round *n*'s fields concatenated with `previous_round_hash` (the `round_hash` of round *n-1*). This produces a hash chain: any modification to a historical round invalidates the hash of every subsequent round.

### 5.2 Verification Procedure

Given a `RoundProof` and the corresponding published `GameManifest` (Section 6), a verifier MUST be able to, without contacting the operator:

1. Recompute the RNG-derived value from `server_seed` (once revealed), `client_seed`, and `nonce`, and confirm it matches `rng.output`.
2. Recompute the mapped outcome from `rng.output` using the published mapping algorithm and confirm it matches `outcome`.
3. Confirm `game_logic_hash` and `paytable_hash` match the values published in the active `GameManifest` for `game_version`.
4. Recompute the paytable lookup from `outcome` and confirm it matches `payout`.
5. Confirm `round_hash` is consistent with `previous_round_hash` and the round's own canonical content.
6. Confirm `signature` validates against the operator's published public key.

A round for which all six checks pass is **Cryptographically Verified** (Trust Level 1, see Section 8).

## 6. Game Manifest

Every game version deployed under CIGP MUST publish a `GameManifest` containing, at minimum: RNG algorithm and version, theoretical RTP, volatility classification, paytable hash, game logic hash, configuration hash, and the jurisdictions under which the game is deployed. See [`schemas/game-manifest.schema.json`](../schemas/game-manifest.schema.json).

The manifest MUST include a `certification.status` field. CIGP conformance MUST NOT be represented as equivalent to, or a substitute for, certification by an accredited independent test laboratory.

## 7. Audit Ledger and Merkle Anchoring

1. Round hashes are appended to a local, append-only ledger as they are produced.
2. At the end of each anchoring interval (operator-configurable), all round hashes in the interval are assembled into a Merkle tree.
3. The resulting Merkle root, together with a timestamp, is published to a public anchor (e.g. a public timestamping service; a blockchain anchor is permitted but not required by this specification).
4. A verifier possessing a single `RoundProof` and its Merkle inclusion path MUST be able to confirm the round is a member of a specific, timestamped, published batch — without requiring disclosure of all other rounds in that batch.

## 8. Trust Levels

| Level | Requirements |
|---|---|
| **0 — Unverified** | No RoundProof available, or verification not attempted. |
| **1 — Cryptographically Verified** | All checks in Section 5.2 pass. |
| **2 — System Verified** | Level 1, plus `game_logic_hash`, `paytable_hash`, and `configuration_hash` all match the currently published `GameManifest`. |
| **3 — Independently Audited** | Level 2, plus confirmation recorded by an independent test laboratory and/or regulator using CIGP evidence. |

Implementations and integrators MUST NOT represent Trust Level 3 as equivalent to possession of a gambling license in any jurisdiction.

## 9. Explicit Non-Claims

Conformant CIGP implementations and any documentation, marketing, or certificate produced under this specification MUST NOT assert that CIGP proves an operator is "honest" or "fair" in an unqualified sense. Conformant claims are limited to the enumerated checks in Sections 5.2, 6, 7, and 8.

## 10. Change Control

This is a `v0.1` draft. Backwards-incompatible changes to the `RoundProof`, `GameManifest`, or hash-chain construction require a new minor version and MUST be accompanied by a migration note and updated test vectors under `test-vectors/`.
