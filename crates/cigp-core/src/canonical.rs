//! Canonical JSON serialization for CIGP objects.
//!
//! CIGP requires that every hashed or signed object be serialized into a
//! single, deterministic byte sequence so that independent implementations
//! (Rust, Python, TypeScript, Julia) compute identical hashes from identical
//! logical content.
//!
//! This module implements a canonicalization scheme compatible with the
//! principles of RFC 8785 (JSON Canonicalization Scheme):
//!
//! - Object keys are sorted lexicographically (byte-wise, by UTF-8 code point).
//! - No insignificant whitespace is emitted.
//! - Numbers are emitted in their minimal, unambiguous form.
//! - Nested objects and arrays are canonicalized recursively.
//!
//! CIGP never serializes floating-point monetary values (see [`crate::money`]),
//! which sidesteps the hardest edge cases of JSON number canonicalization.

use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

use crate::error::CigpError;

/// Convert an arbitrary `serde_json::Value` into its canonical byte
/// representation.
pub fn canonicalize(value: &Value) -> Vec<u8> {
    let mut out = Vec::new();
    write_canonical(value, &mut out);
    out
}

/// Canonicalize a serializable value directly to bytes.
pub fn canonicalize_serializable<T: serde::Serialize>(value: &T) -> Result<Vec<u8>, CigpError> {
    let v = serde_json::to_value(value).map_err(CigpError::Serialization)?;
    Ok(canonicalize(&v))
}

/// SHA-256 of a value's canonical byte representation, returned as lowercase hex.
pub fn canonical_hash<T: serde::Serialize>(value: &T) -> Result<String, CigpError> {
    let bytes = canonicalize_serializable(value)?;
    let digest = Sha256::digest(&bytes);
    Ok(format!("sha256:{}", hex::encode(digest)))
}

fn write_canonical(value: &Value, out: &mut Vec<u8>) {
    match value {
        Value::Null => out.extend_from_slice(b"null"),
        Value::Bool(b) => out.extend_from_slice(if *b { b"true" } else { b"false" }),
        Value::Number(n) => {
            // CIGP forbids floating point monetary values, but canonical
            // serialization must still behave deterministically for any
            // integer or non-monetary numeric field present in an object.
            out.extend_from_slice(n.to_string().as_bytes());
        }
        Value::String(s) => write_json_string(s, out),
        Value::Array(items) => {
            out.push(b'[');
            for (i, item) in items.iter().enumerate() {
                if i > 0 {
                    out.push(b',');
                }
                write_canonical(item, out);
            }
            out.push(b']');
        }
        Value::Object(map) => {
            let sorted: BTreeMap<&String, &Value> = map.iter().collect();
            out.push(b'{');
            for (i, (k, v)) in sorted.iter().enumerate() {
                if i > 0 {
                    out.push(b',');
                }
                write_json_string(k, out);
                out.push(b':');
                write_canonical(v, out);
            }
            out.push(b'}');
        }
    }
}

fn write_json_string(s: &str, out: &mut Vec<u8>) {
    out.push(b'"');
    for c in s.chars() {
        match c {
            '"' => out.extend_from_slice(b"\\\""),
            '\\' => out.extend_from_slice(b"\\\\"),
            '\n' => out.extend_from_slice(b"\\n"),
            '\r' => out.extend_from_slice(b"\\r"),
            '\t' => out.extend_from_slice(b"\\t"),
            c if (c as u32) < 0x20 => {
                out.extend_from_slice(format!("\\u{:04x}", c as u32).as_bytes());
            }
            c => {
                let mut buf = [0u8; 4];
                out.extend_from_slice(c.encode_utf8(&mut buf).as_bytes());
            }
        }
    }
    out.push(b'"');
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn key_order_is_irrelevant() {
        let a = json!({"b": 1, "a": 2});
        let b = json!({"a": 2, "b": 1});
        assert_eq!(canonicalize(&a), canonicalize(&b));
    }

    #[test]
    fn nested_objects_are_sorted_recursively() {
        let v = json!({"z": {"y": 1, "x": 2}, "a": [3, {"c": 1, "b": 2}]});
        let bytes = canonicalize(&v);
        let s = String::from_utf8(bytes).unwrap();
        assert_eq!(s, r#"{"a":[3,{"b":2,"c":1}],"z":{"x":2,"y":1}}"#);
    }

    #[test]
    fn strings_are_escaped() {
        let v = json!({"k": "line1\nline2\t\"quoted\""});
        let bytes = canonicalize(&v);
        let s = String::from_utf8(bytes).unwrap();
        assert_eq!(s, r#"{"k":"line1\nline2\t\"quoted\""}"#);
    }

    #[test]
    fn canonical_hash_is_deterministic() {
        let a = json!({"b": 1, "a": 2});
        let b = json!({"a": 2, "b": 1});
        assert_eq!(canonical_hash(&a).unwrap(), canonical_hash(&b).unwrap());
    }

    #[test]
    fn canonicalization_matches_cross_language_fixture() {
        let value = json!({"z": [3, {"b": 2, "a": 1}], "a": "cigp"});
        assert_eq!(
            String::from_utf8(canonicalize(&value)).unwrap(),
            r#"{"a":"cigp","z":[3,{"a":1,"b":2}]}"#
        );
        assert_eq!(
            canonical_hash(&value).unwrap(),
            "sha256:181afed0f3aba1ad21b26fefb99455dd6d252ccf38c77f69cf6659b8c53c67f7"
        );
    }
}
