# RoundProof

`RoundProof` is the signed evidence object for one gaming-system event. Its Rust representation is in `crates/cigp-core`; the hash construction is implemented in `crates/cigp-proof`.

The round hash covers every proof field except `round_hash`, `signature`, and the optional revealed `server_seed`. This permits a verifier to validate the same pre-reveal object before the secret is disclosed. Once revealed, the commitment and HMAC derivation checks bind the seed to the proof.

Money is represented as an ISO currency code plus integer minor units. Floating-point monetary values are not part of the protocol.
