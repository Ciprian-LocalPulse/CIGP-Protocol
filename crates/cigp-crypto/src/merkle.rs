//! Merkle tree batching over round hashes, with inclusion proof generation
//! and verification (CIGP §15).
//!
//! Leaves are domain-separated from internal nodes (`0x00` vs `0x01` prefix)
//! to prevent second-preimage attacks where a leaf is misinterpreted as an
//! internal node or vice versa.

use crate::hashing::sha256_bytes;

const LEAF_PREFIX: u8 = 0x00;
const NODE_PREFIX: u8 = 0x01;

fn hash_leaf(data: &[u8]) -> [u8; 32] {
    let mut buf = vec![LEAF_PREFIX];
    buf.extend_from_slice(data);
    sha256_bytes(&buf)
}

fn hash_node(left: &[u8; 32], right: &[u8; 32]) -> [u8; 32] {
    let mut buf = vec![NODE_PREFIX];
    buf.extend_from_slice(left);
    buf.extend_from_slice(right);
    sha256_bytes(&buf)
}

/// A step in a Merkle inclusion proof: a sibling hash and which side it sits on.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProofStep {
    pub sibling: [u8; 32],
    pub sibling_is_left: bool,
}

/// A complete Merkle tree over a batch of round hashes (as raw bytes, e.g.
/// each round's `round_hash` field decoded from hex).
pub struct MerkleTree {
    levels: Vec<Vec<[u8; 32]>>,
}

#[derive(Debug, thiserror::Error)]
pub enum MerkleError {
    #[error("cannot build a Merkle tree over an empty leaf set")]
    EmptyLeaves,
    #[error("leaf index {0} out of range")]
    IndexOutOfRange(usize),
}

impl MerkleTree {
    /// Build a tree from raw leaf data. Odd levels are completed by
    /// duplicating the final node, per common Merkle tree convention.
    pub fn build(leaves: &[Vec<u8>]) -> Result<Self, MerkleError> {
        if leaves.is_empty() {
            return Err(MerkleError::EmptyLeaves);
        }
        let mut level: Vec<[u8; 32]> = leaves.iter().map(|l| hash_leaf(l)).collect();
        let mut levels = vec![level.clone()];

        while level.len() > 1 {
            let mut next = Vec::with_capacity((level.len() + 1) / 2);
            let mut i = 0;
            while i < level.len() {
                if i + 1 < level.len() {
                    next.push(hash_node(&level[i], &level[i + 1]));
                } else {
                    // Odd node out: duplicate.
                    next.push(hash_node(&level[i], &level[i]));
                }
                i += 2;
            }
            levels.push(next.clone());
            level = next;
        }

        Ok(MerkleTree { levels })
    }

    pub fn root(&self) -> [u8; 32] {
        *self.levels.last().unwrap().last().unwrap()
    }

    pub fn root_hex(&self) -> String {
        format!("sha256:{}", hex::encode(self.root()))
    }

    pub fn leaf_count(&self) -> usize {
        self.levels[0].len()
    }

    /// Generate an inclusion proof for the leaf at `index`.
    pub fn prove(&self, index: usize) -> Result<Vec<ProofStep>, MerkleError> {
        if index >= self.leaf_count() {
            return Err(MerkleError::IndexOutOfRange(index));
        }
        let mut proof = Vec::new();
        let mut idx = index;
        for level in &self.levels[..self.levels.len() - 1] {
            let sibling_idx = if idx % 2 == 0 { idx + 1 } else { idx - 1 };
            let sibling = if sibling_idx < level.len() {
                level[sibling_idx]
            } else {
                level[idx] // duplicated odd node
            };
            proof.push(ProofStep {
                sibling,
                sibling_is_left: idx % 2 == 1,
            });
            idx /= 2;
        }
        Ok(proof)
    }
}

/// Verify that `leaf_data` is included in the tree committed to by `root`,
/// given an inclusion proof.
pub fn verify_inclusion(leaf_data: &[u8], proof: &[ProofStep], root: &[u8; 32]) -> bool {
    let mut hash = hash_leaf(leaf_data);
    for step in proof {
        hash = if step.sibling_is_left {
            hash_node(&step.sibling, &hash)
        } else {
            hash_node(&hash, &step.sibling)
        };
    }
    &hash == root
}

#[cfg(test)]
mod tests {
    use super::*;

    fn leaves(n: usize) -> Vec<Vec<u8>> {
        (0..n).map(|i| format!("round-{i}").into_bytes()).collect()
    }

    #[test]
    fn single_leaf_tree() {
        let l = leaves(1);
        let tree = MerkleTree::build(&l).unwrap();
        let proof = tree.prove(0).unwrap();
        assert!(verify_inclusion(&l[0], &proof, &tree.root()));
    }

    #[test]
    fn even_leaf_count_all_inclusion_proofs_verify() {
        let l = leaves(8);
        let tree = MerkleTree::build(&l).unwrap();
        for i in 0..l.len() {
            let proof = tree.prove(i).unwrap();
            assert!(verify_inclusion(&l[i], &proof, &tree.root()));
        }
    }

    #[test]
    fn odd_leaf_count_all_inclusion_proofs_verify() {
        let l = leaves(7);
        let tree = MerkleTree::build(&l).unwrap();
        for i in 0..l.len() {
            let proof = tree.prove(i).unwrap();
            assert!(verify_inclusion(&l[i], &proof, &tree.root()));
        }
    }

    #[test]
    fn tampered_leaf_fails_inclusion() {
        let l = leaves(4);
        let tree = MerkleTree::build(&l).unwrap();
        let proof = tree.prove(1).unwrap();
        assert!(!verify_inclusion(b"round-99", &proof, &tree.root()));
    }

    #[test]
    fn out_of_range_index_errors() {
        let l = leaves(3);
        let tree = MerkleTree::build(&l).unwrap();
        assert!(tree.prove(3).is_err());
    }
}
