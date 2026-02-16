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
  /// Creates a new, empty R1CS instance.
  pub fn new() -> Self {
    R1CS{
      variables: Vec::new(),
      constraints: Vec::new()
    }
  }

  /// Adds a constraint to the R1CS with the given left, right, output, and operation.
  pub fn add_constraints(&mut self, left: Vec<(Variable, BigInt)>, right: Vec<(Variable, BigInt)>, output: Vec<(Variable, BigInt)>, operation: Operation) {
    let constraint = Constraint{
      left,
      right,
      output,
      operation
    };
    self.constraints.push(constraint);
  }

  /// Checks if all constraints are satisfied by the current variable assignments.
  /// Returns true if all constraints hold, false otherwise.
  /// The apply_hash closure is used for Hash operations.
  pub fn is_satisfied<F>(&self, apply_hash: F) -> bool
  where F: Fn(&BigInt, &BigInt) -> BigInt // a closure that applies the hash
  {
    for constraint in &self.constraints {
      let left_value: BigInt = constraint.left.iter().map(|(var, coeff)| &var.value * coeff).sum();
      let right_value: BigInt = constraint.right.iter().map(|(var, coeff)| &var.value * coeff).sum();
      let output_value: BigInt = constraint.output.iter().map(|(var, coeff)| &var.value * coeff).sum();

      match constraint.operation {
        Operation::Add => {
          if left_value.clone() + right_value.clone() != output_value.clone() {
            print!("Add constraint not satisfied: {:?} + {:?} != {:?}", left_value, right_value, output_value);
            return false;
          }
        },
        Operation::Mul => {
          if left_value.clone() * right_value.clone() != output_value.clone() {
            print!("Mul constraint not satisfied: {:?} * {:?} != {:?}", left_value, right_value, output_value);
            return false;
          }
        },
        Operation::Hash => {
          // For a hash operation, we expect the output to be the result of applying the hash function
          let expected_output = apply_hash(&left_value, &right_value);
          if expected_output != output_value {
            print!("Hash constraint not satisfied: expected {:?}, got {:?}", expected_output, output_value);
            return false;
          }
        }
      }
    }
    true
  }

  /// Serializes and saves the entire R1CS structure to a binary file.
  pub fn save_to_binary(&self, filename: &str){
    let mut file = File::create(filename).expect("Unable to create the file");
    let data = bincode::serialize(self).expect("Here failed to serialize R1CS");
    file.write_all(&data).expect("Unable to write data to the file");
  }
}

