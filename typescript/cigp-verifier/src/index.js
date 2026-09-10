import { createHash } from "node:crypto";

export function sha256Utf8(value) {
  return `sha256:${createHash("sha256").update(value, "utf8").digest("hex")}`;
}

export function isCigpVersionSupported(version) {
  return version === "0.1";
}
