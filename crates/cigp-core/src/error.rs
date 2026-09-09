use thiserror::Error;

#[derive(Debug, Error)]
pub enum CigpError {
    #[error("serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("hash mismatch: expected {expected}, got {actual}")]
    HashMismatch { expected: String, actual: String },

    #[error("invalid field: {0}")]
    InvalidField(String),

    #[error("unsupported protocol version: {0}")]
    UnsupportedVersion(String),
}
