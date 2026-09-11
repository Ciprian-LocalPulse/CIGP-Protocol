import { createHash, createHmac } from "node:crypto";

export function sha256Utf8(value) {
  return `sha256:${createHash("sha256").update(value, "utf8").digest("hex")}`;
}

export function isCigpVersionSupported(version) {
  return version === "0.1";
}

export function canonicalJson(value) {
  if (value === null || typeof value !== "object") {
    return JSON.stringify(value);
  }
  if (Array.isArray(value)) {
    return `[${value.map(canonicalJson).join(",")}]`;
  }
  return `{${Object.keys(value)
    .sort()
    .map((key) => `${JSON.stringify(key)}:${canonicalJson(value[key])}`)
    .join(",")}}`;
}

export function canonicalSha256Hex(value) {
  return createHash("sha256").update(canonicalJson(value), "utf8").digest("hex");
}

export function deriveReferenceRng(serverSeedHex, clientSeed, nonce) {
  if (!Number.isSafeInteger(nonce) || nonce < 0) {
    throw new RangeError("nonce must be a non-negative safe integer");
  }
  const key = /^(?:[0-9a-f]{2})+$/i.test(serverSeedHex)
    ? Buffer.from(serverSeedHex, "hex")
    : Buffer.from(serverSeedHex, "utf8");
  const nonceBytes = Buffer.alloc(8);
  nonceBytes.writeBigUInt64BE(BigInt(nonce));
  const message = Buffer.concat([Buffer.from(clientSeed, "utf8"), Buffer.from([0x1f]), nonceBytes]);
  return createHmac("sha256", key).update(message).digest("hex");
}
