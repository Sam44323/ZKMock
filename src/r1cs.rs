use num_bigint::BigInt;
use serde::{Deserialize, Serialize};  
use std::fs::File;
use std::io::{Write};


/// Implements serialization, deserialization, and debug printing for Variable.
/// - Serialize: Enables converting Variable to formats like JSON or binary.
/// - Deserialize: Enables creating Variable from formats like JSON or binary.
/// - Debug: Enables printing Variable with `{:?}` for debugging.
#[derive(Serialize, Deserialize, Debug)]
pub struct Variable{
  pub index: usize,
  pub value: BigInt
}

#[derive(Serialize, Deserialize)]
pub enum Operation{
  Add,
  Mul,
  Hash
}

/**
 * Example
 * 
 * For the operation of x + y = z,
 * 
 * -> left is x
 * -> right is y
 * -> operation is '+'
 * -> output is z
 */

#[derive(Serialize, Deserialize)]
pub struct Constraint{
  pub left: Vec<(Variable, BigInt)>,
  pub right: Vec<(Variable, BigInt)>,
  pub output: Vec<(Variable, BigInt)>,
  pub operation: Operation
}

#[derive(Serialize, Deserialize)]
pub struct R1CS{
  pub variables: Vec<Variable>,
  pub constraints: Vec<Constraint>
}

impl R1CS {
  pub fn new() -> Self {
    R1CS{
      variables: Vec::new(),
      constraints: Vec::new()
    }
  }

  pub fn add_constraints(&mut self, left: Vec<(Variable, BigInt)>, right: Vec<(Variable, BigInt)>, output: Vec<(Variable, BigInt)>, operation: Operation) {
    let constraint = Constraint{
      left,
      right,
      output,
      operation
    };
    self.constraints.push(constraint);
  }
}

