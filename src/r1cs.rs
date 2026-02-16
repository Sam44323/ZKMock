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
 * let x = Variable { index: 0, value: BigInt::from(5) };
 * let y = Variable { index: 1, value: BigInt::from(7) };
 * let z = Variable { index: 2, value: BigInt::from(12) };
 * 
 * let constraint = Constraint {
 * left: vec![(x.clone(), x.value.clone())],
 * right: vec![(y.clone(), y.value.clone())],
 * output: vec![(z.clone(), z.value.clone())],
 * operation: Operation::Add,
 * };
 */

#[derive(Serialize, Deserialize)]
pub struct Constraint{
  pub left: Vec<(Variable, BigInt)>,
  pub right: Vec<(Variable, BigInt)>,
  pub output: Vec<(Variable, BigInt)>,
  pub operation: Operation
}