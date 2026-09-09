//! CIGP domain types: [`RoundProof`], [`GameManifest`], and their nested
//! structures, as specified in `spec/round-proof.md` and
//! `spec/game-manifest.md`.

use crate::money::Money;
use serde::{Deserialize, Serialize};

/// Current CIGP protocol version implemented by this crate.
pub const CIGP_VERSION: &str = "0.1";

/// Reference cryptographic profile identifier for the commit-reveal RNG
/// derivation defined in `spec/cryptographic-profile.md`.
pub const REFERENCE_RNG_PROFILE: &str = "CIGP-REFERENCE-HMAC-SHA256";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RngEvidence {
    pub algorithm: String,
    pub version: String,
    /// Hex-encoded RNG output derived from the commit-reveal scheme.
    pub output: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MappingEvidence {
    pub algorithm: String,
    pub version: String,
    /// `sha256:<hex>` hash of the parameters used to map RNG output to a
    /// game outcome (e.g. reel strips, paytable weighting table).
    pub parameters_hash: String,
}

/// A strongly-typed, protocol-versioned cryptographic proof for a single
/// gaming round. Every field participates in the round hash and is
/// independently checkable by [`crate::canonical`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoundProof {
    pub cigp_version: String,
    pub round_id: String,
    pub operator_id: String,
    pub game_id: String,
    pub game_version: String,

    pub bet: Money,
    pub payout: Money,

    /// SHA-256 commitment to the server seed: `sha256(server_seed)`.
    pub server_commitment: String,
    /// Revealed only after outcome determination (post-round reveal phase).
    pub server_seed: Option<String>,
    pub client_seed: String,
    pub nonce: u64,

    pub rng: RngEvidence,
    pub mapping: MappingEvidence,

    /// Free-form, game-specific outcome payload (reel stops, card draws, ...).
    pub outcome: serde_json::Value,

    pub paytable_hash: String,
    pub game_logic_hash: String,
    pub configuration_hash: String,

    /// Hash of the previous round's `round_hash` in the same ledger chain.
    /// `None` (or the literal genesis marker) for the first round.
    pub previous_round_hash: String,
    /// Canonical hash of this round's own content (all fields above),
    /// computed and populated by [`crate::proof`].
    pub round_hash: String,
    /// Ed25519 signature over `round_hash`, hex-encoded.
    pub signature: String,

    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RngProfile {
    pub profile: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mathematics {
    /// Declared theoretical RTP as a fixed-point decimal string, e.g. "0.960000".
    pub theoretical_rtp: String,
    pub volatility: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HashRef {
    pub hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameManifest {
    pub cigp_version: String,
    pub game_id: String,
    pub game_version: String,
    pub rng: RngProfile,
    pub mathematics: Mathematics,
    pub paytable: HashRef,
    pub game_logic: HashRef,
    pub configuration: HashRef,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CheckResult {
    Pass,
    Fail,
}

impl CheckResult {
    pub fn from_bool(b: bool) -> Self {
        if b {
            CheckResult::Pass
        } else {
            CheckResult::Fail
        }
    }
    pub fn is_pass(&self) -> bool {
        matches!(self, CheckResult::Pass)
    }
}

impl std::fmt::Display for CheckResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", if self.is_pass() { "PASS" } else { "FAIL" })
    }
}
