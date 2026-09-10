//! Ed25519 signing and verification over round hashes.

use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use rand_core::OsRng;

#[derive(Debug, thiserror::Error)]
pub enum SignatureError {
    #[error("invalid hex encoding: {0}")]
    Hex(#[from] hex::FromHexError),
    #[error("invalid key or signature bytes")]
    InvalidBytes,
    #[error("signature verification failed")]
    VerificationFailed,
}

/// An Ed25519 keypair used by a CIGP operator to sign round hashes.
pub struct OperatorKeypair {
    signing_key: SigningKey,
}

impl OperatorKeypair {
    /// Generate a fresh keypair. In production, operator keys must be
    /// managed by an HSM or equivalent; this is provided for the reference
    /// implementation and test vectors only.
    pub fn generate() -> Self {
        let signing_key = SigningKey::generate(&mut OsRng);
        OperatorKeypair { signing_key }
    }

    pub fn from_seed_hex(seed_hex: &str) -> Result<Self, SignatureError> {
        let bytes = hex::decode(seed_hex)?;
        let arr: [u8; 32] = bytes.try_into().map_err(|_| SignatureError::InvalidBytes)?;
        Ok(OperatorKeypair {
            signing_key: SigningKey::from_bytes(&arr),
        })
    }

    pub fn public_key_hex(&self) -> String {
        hex::encode(self.signing_key.verifying_key().to_bytes())
    }

    pub fn secret_key_hex(&self) -> String {
        hex::encode(self.signing_key.to_bytes())
    }

    /// Sign a round hash string (the `round_hash` field, e.g.
    /// `"sha256:<hex>"`), returning a hex-encoded signature.
    pub fn sign(&self, round_hash: &str) -> String {
        let sig: Signature = self.signing_key.sign(round_hash.as_bytes());
        hex::encode(sig.to_bytes())
    }
}

/// Verify a hex-encoded Ed25519 signature over a round hash, given the
/// operator's hex-encoded public key.
pub fn verify(
    public_key_hex: &str,
    round_hash: &str,
    signature_hex: &str,
) -> Result<(), SignatureError> {
    let pk_bytes = hex::decode(public_key_hex)?;
    let pk_arr: [u8; 32] = pk_bytes
        .try_into()
        .map_err(|_| SignatureError::InvalidBytes)?;
    let verifying_key =
        VerifyingKey::from_bytes(&pk_arr).map_err(|_| SignatureError::InvalidBytes)?;

    let sig_bytes = hex::decode(signature_hex)?;
    let sig_arr: [u8; 64] = sig_bytes
        .try_into()
        .map_err(|_| SignatureError::InvalidBytes)?;
    let signature = Signature::from_bytes(&sig_arr);

    verifying_key
        .verify(round_hash.as_bytes(), &signature)
        .map_err(|_| SignatureError::VerificationFailed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sign_and_verify_round_trip() {
        let kp = OperatorKeypair::generate();
        let sig = kp.sign("sha256:deadbeef");
        assert!(verify(&kp.public_key_hex(), "sha256:deadbeef", &sig).is_ok());
    }

    #[test]
    fn tampered_message_fails_verification() {
        let kp = OperatorKeypair::generate();
        let sig = kp.sign("sha256:deadbeef");
        assert!(verify(&kp.public_key_hex(), "sha256:tampered", &sig).is_err());
    }

    #[test]
    fn wrong_key_fails_verification() {
        let kp = OperatorKeypair::generate();
        let other = OperatorKeypair::generate();
        let sig = kp.sign("sha256:deadbeef");
        assert!(verify(&other.public_key_hex(), "sha256:deadbeef", &sig).is_err());
    }
}
