import test from "node:test";
import assert from "node:assert/strict";
import fs from "node:fs";
import { canonicalJson, canonicalSha256Hex, deriveReferenceRng, isCigpVersionSupported, sha256Utf8 } from "../src/index.js";

test("supports the current protocol and known hash vector", () => {
  assert.equal(isCigpVersionSupported("0.1"), true);
  assert.equal(sha256Utf8(""), "sha256:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855");
});

test("matches canonicalization and HMAC cross-language vector", () => {
  const vector = JSON.parse(fs.readFileSync("../../test-vectors/canonical-hmac-v1.json", "utf8"));
  assert.equal(canonicalJson(vector.canonicalization.input), vector.canonicalization.canonical_json);
  assert.equal(canonicalSha256Hex(vector.canonicalization.input), vector.canonicalization.sha256_hex);
  assert.equal(
    deriveReferenceRng(
      vector.hmac_derivation.server_seed_hex,
      vector.hmac_derivation.client_seed,
      vector.hmac_derivation.nonce,
    ),
    vector.hmac_derivation.rng_output_hex,
  );
});
