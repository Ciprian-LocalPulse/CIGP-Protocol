import test from "node:test";
import assert from "node:assert/strict";
import { isCigpVersionSupported, sha256Utf8 } from "../src/index.js";

test("supports the current protocol and known hash vector", () => {
  assert.equal(isCigpVersionSupported("0.1"), true);
  assert.equal(sha256Utf8(""), "sha256:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855");
});
