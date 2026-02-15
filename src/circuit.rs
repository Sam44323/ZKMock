use std::io::Write;
use num_bigint::BigInt;
use crate::hash_functions::HashFunction;
use crate::r1cs::{Operation, R1CS, Variable};

/**
 * zk-proof operations happens through the usage of gates
 */
pub enum Gate{
  Add(usize, usize, usize), // Add: input-A, input-B, output
  Mul(usize, usize, usize), // Mul: input-B, input-B, output
  Hash(usize, usize, usize) // Hash: input-A, input-B, output
}