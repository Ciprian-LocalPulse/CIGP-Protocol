# CIGP Reference Cryptographic Profile

Version: `CIGP-REFERENCE-HMAC-SHA256` v1

This profile is the reproducible reference construction for CIGP v0.1. It is not a claim that every certified gaming platform must use this construction.

- Commitment: `sha256:<hex>` over the UTF-8 bytes of the revealed server seed.
- Round derivation: HMAC-SHA-256 with the decoded server seed as key and `client_seed || 0x1f || nonce.to_be_bytes()` as message.
- Signatures: Ed25519 over the `round_hash` string.
- Merkle nodes: SHA-256 with domain prefixes `0x00` for leaves and `0x01` for internal nodes.
- Canonicalization: sorted JSON object keys, compact JSON, UTF-8 output.

Server seeds are secret before reveal. Production key material belongs in an HSM or equivalent secret-management system and must never be committed to this repository.
