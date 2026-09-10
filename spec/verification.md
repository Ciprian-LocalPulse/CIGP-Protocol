# Verification

Independent verification recomputes the round hash, Ed25519 signature, server commitment, and revealed-seed derivation. A verifier returns `VALID` only when every required check passes.

CLI exit codes:

- `0`: valid proof
- `1`: cryptographically invalid proof
- `2`: malformed input or missing verification material
- `3`: unsupported protocol version
- `4`: internal verification error
