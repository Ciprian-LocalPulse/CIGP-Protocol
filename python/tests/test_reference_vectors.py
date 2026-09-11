import json
from pathlib import Path

from cigp_anomaly.reference import canonical_json, canonical_sha256_hex, derive_reference_rng


VECTOR_PATH = Path(__file__).parents[2] / "test-vectors" / "canonical-hmac-v1.json"


def test_canonicalization_vector_matches_reference():
    vector = json.loads(VECTOR_PATH.read_text(encoding="utf-8"))
    case = vector["canonicalization"]
    assert canonical_json(case["input"]) == case["canonical_json"]
    assert canonical_sha256_hex(case["input"]) == case["sha256_hex"]


def test_hmac_vector_matches_reference():
    vector = json.loads(VECTOR_PATH.read_text(encoding="utf-8"))
    case = vector["hmac_derivation"]
    assert derive_reference_rng(case["server_seed_hex"], case["client_seed"], case["nonce"]) == case["rng_output_hex"]
