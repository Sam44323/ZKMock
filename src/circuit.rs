use std::io::Write;
use num_bigint::BigInt;
use crate::hash_functions::HashFunction;
use crate::r1cs::{Operation, R1CS, Variable};

/**
 * @note
 * zk-proof operations happens through the usage of gates
 */
pub enum Gate{
  Add(usize, usize, usize), // Add: input-A, input-B, output
  Mul(usize, usize, usize), // Mul: input-B, input-B, output
  Hash(usize, usize, usize) // Hash: input-A, input-B, output
}

pub struct Circuit{
  hash_function: Option<Box<dyn HashFunction>>,
  inputs: Vec<BigInt>,
  gates: Vec<Gate>,
  outputs: Vec<BigInt>
}

impl Circuit {
  pub fn new(hash_function: Option<Box<dyn HashFunction>>) -> Self {
    Circuit{
      hash_function,
      inputs: Vec::new(),
      gates: Vec::new(),
      outputs: Vec::new()
    }
  }

  pub fn add_input(&mut self, input: BigInt) -> usize {
    let index = self.inputs.len();
    self.inputs.push(input);
    index
  }

  pub fn add_gate(&mut self, gate: Gate) {
    self.gates.push(gate);
  }

  pub fn add_output(&mut self, output: BigInt) {
    self.outputs.push(output);
  }
}