"""Deterministic anomaly research helpers; an anomaly means investigate, not fraud."""

from .detectors import z_score, classify
from .reference import canonical_json, canonical_sha256_hex, derive_reference_rng

__all__ = ["z_score", "classify", "canonical_json", "canonical_sha256_hex", "derive_reference_rng"]
