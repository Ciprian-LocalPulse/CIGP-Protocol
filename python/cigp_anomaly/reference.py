"""Cross-language helpers for the CIGP reference cryptographic profile.

These helpers reproduce public test vectors. They are not a replacement for
the Rust verifier and do not handle production secret management.
"""

from __future__ import annotations

import hashlib
import hmac
import json
from typing import Any


def canonical_json(value: Any) -> str:
    """Encode the JSON subset used by CIGP vectors with sorted keys and no whitespace."""
    return json.dumps(value, ensure_ascii=False, separators=(",", ":"), sort_keys=True)


def canonical_sha256_hex(value: Any) -> str:
    return hashlib.sha256(canonical_json(value).encode("utf-8")).hexdigest()


def derive_reference_rng(server_seed_hex: str, client_seed: str, nonce: int) -> str:
    if nonce < 0 or nonce > (2**64 - 1):
        raise ValueError("nonce must fit in an unsigned 64-bit integer")
    try:
        key = bytes.fromhex(server_seed_hex)
    except ValueError:
        key = server_seed_hex.encode("utf-8")
    message = client_seed.encode("utf-8") + b"\x1f" + nonce.to_bytes(8, "big")
    return hmac.new(key, message, hashlib.sha256).hexdigest()
