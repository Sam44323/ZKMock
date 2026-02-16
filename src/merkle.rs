use num_bigint::BigInt;
use crate::hash_functions::HashFunction;

/// A Merkle tree for cryptographic data verification.
/// 
/// Merkle trees allow efficient proof that data is part of a larger dataset
/// by hashing pairs of nodes recursively to produce a single root hash.
/// 
/// Generic over `H`: any type implementing the `HashFunction` trait.
pub struct MerkleTree<H: HashFunction> {
    /// The root hash - cryptographic commitment to all leaf values
    pub root: BigInt,
    
    /// The original leaf values at the bottom of the tree
    pub leaves: Vec<BigInt>,
    
    /// Hash function used to compute internal node hashes
    hash_function: H
}

impl<H: HashFunction> MerkleTree<H> {
    /// Constructs a new Merkle tree from leaf values.
    /// 
    /// Computes the root hash by recursively hashing pairs of nodes
    /// from the bottom up until a single root remains.
    /// 
    /// # Arguments
    /// * `leaves` - Vector of leaf node values
    /// * `hash_function` - Hash function instance to use
    pub fn new(leaves: Vec<BigInt>, hash_function: H) -> Self {
        // Compute root by passing references (& prevents moving ownership)
        let root = MerkleTree::compute_root(&leaves, &hash_function);
        
        MerkleTree { root, leaves, hash_function }
    }

    /// Generates a Merkle proof (authentication path) for a specific leaf.
    /// 
    /// Returns the sibling hashes needed to recompute the root from the leaf,
    /// allowing anyone to verify the leaf is part of the tree without seeing all data.
    /// 
    /// # Arguments
    /// * `index` - Zero-based index of the leaf to prove
    /// 
    /// # Returns
    /// Vector of (sibling_hash, is_right_sibling) tuples:
    /// - `sibling_hash`: The hash of the sibling node
    /// - `is_right_sibling`: true if sibling is on the right (you're left child)
    pub fn merkle_path(&self, index: usize) -> Vec<(BigInt, bool)> {
        let mut path = Vec::new();
        let mut current_index = index;
        let mut nodes = self.leaves.clone();  // Working copy of nodes

        // Traverse up the tree level by level until we reach the root
        while nodes.len() > 1 {
            // Build the next level up by hashing pairs of nodes
            let next_level: Vec<BigInt> = nodes
                .chunks(2)  // Split current level into pairs
                .map(|chunk| {
                    if chunk.len() == 2 {
                        // Hash the pair together
                        self.hash_function.hash(&chunk[0], &chunk[1])
                    } else {
                        // Odd node (no pair), promote it unchanged
                        chunk[0].clone()
                    }
                })
                .collect();

            // Calculate sibling index: if we're even, sibling is +1; if odd, sibling is -1
            let sibling_index = if current_index % 2 == 0 { 
                current_index + 1 
            } else { 
                current_index - 1 
            };
            
            // Add sibling to proof path if it exists
            if sibling_index < nodes.len() {
                path.push((
                    nodes[sibling_index].clone(),
                    current_index % 2 == 0  // true means we're left child
                ));
            }

            // Move to parent index (integer division by 2)
            current_index /= 2;
            nodes = next_level;
        }
        
        path
    }

    /// Computes the Merkle root by building the tree bottom-up.
    /// 
    /// Repeatedly hashes pairs of nodes to create parent nodes,
    /// continuing until only a single root node remains.
    /// 
    /// # Arguments
    /// * `leaves` - Reference to leaf values
    /// * `hash_function` - Reference to hash function
    fn compute_root(leaves: &Vec<BigInt>, hash_function: &H) -> BigInt {
        let mut nodes = leaves.clone();
        
        // Build successive levels by hashing pairs until one node remains
        while nodes.len() > 1 {
            nodes = nodes
                .chunks(2)  // Process in pairs
                .map(|chunk| {
                    if chunk.len() == 2 {
                        // Hash the pair
                        hash_function.hash(&chunk[0], &chunk[1])
                    } else {
                        // Single node (odd count), promote unchanged
                        chunk[0].clone()
                    }
                })
                .collect();
        }
        
        // Return the final root node
        nodes[0].clone()
    }
}