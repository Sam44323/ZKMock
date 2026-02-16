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
  /// Creates a new Circuit with an optional hash function.
  /// Initializes empty inputs, gates, and outputs.
  pub fn new(hash_function: Option<Box<dyn HashFunction>>) -> Self {
    Circuit{
      hash_function,
      inputs: Vec::new(),
      gates: Vec::new(),
      outputs: Vec::new()
    }
  }

  /// Adds an input value to the circuit and returns its index.
  pub fn add_input(&mut self, input: BigInt) -> usize {
    let index = self.inputs.len();
    self.inputs.push(input);
    index
  }

  /// Retrieves a reference to the input value at the given index, if it exists.
  pub fn get_input(&self, index: usize) -> Option<&BigInt> {
    self.inputs.get(index)
  }
  

  /// Adds a gate (operation) to the circuit.
  pub fn add_gate(&mut self, gate: Gate) {
    self.gates.push(gate);
  }

  /// Adds an output value to the circuit.
  pub fn add_output(&mut self, output: BigInt) {
    self.outputs.push(output);
  }

  /// Applies the circuit's hash function to two BigInt values.
  /// Panics if no hash function is defined.
  pub fn apply_hash(&self, a: &BigInt, b: &BigInt) -> BigInt {
    if let Some(ref hash_function) = self.hash_function {
      hash_function.hash(a, b)
    } else {
      // the error handling
      panic!("Hash function not defined for this circuit");
    }
  }

  /**
   * Converts the circuit's gates and inputs into R1CS constraints and prepares for proof-generation.
   *
   * Steps:
   * 1. Initializes a new R1CS instance.
   * 2. Converts all circuit inputs into R1CS variables with unique-indices.
   * 3. Iterates over each gate in the circuit and translates it into an R1CS constraint:
   *    - Add gate: Enforces input[a] + input[b] = input[output].
   *    - Mul gate: Enforces input[a] * input[b] = input[output].
   *    - Hash gate: Computes hash(input[a], input[b]) and enforces it equals output.
   * 4. Each constraint is added to the R1CS system for later proof generation or verification.
   *
   * Arguments:
   * - proof_file: Path to the file where the proof (or R1CS data) will be stored (not implemented in this snippet).
   * 
   * order of the gates in self.gates is synonymous to the opcodes system in EVM and thus it mimics how the entire program was written in DSL(let's say circom) during the compilation-phase
   * 
   * @todo >1-degree polyomials handling doesn't function yet for this method as it doesn't handle the intermediate variables for the operations as of yet
   */
  pub fn generate_proof(&self, proof_file: &str) {
    let mut r1cs = R1CS::new();
    r1cs.variables = self.inputs.iter().enumerate().map(|(i, input)| Variable { index: i, value: input.clone() }).collect();

    // Creating the R1CS constraints based on the gates defined in the circuit
    for gate in &self.gates {
      match gate {
        // For an Add gate, create a constraint enforcing input[a] + input[b] = input[output]
        Gate::Add(a, b, output) => {
          r1cs.add_constraints(
            vec![(Variable { index: *a, value: self.inputs[*a].clone() }, BigInt::from(1))],
            vec![(Variable { index: *b, value: self.inputs[*b].clone() }, BigInt::from(1))],
            vec![(Variable { index: *output, value: self.inputs[*output].clone() }, BigInt::from(1))],
            Operation::Add
          );
        }
        // For a Mul gate, create a constraint enforcing input[a] * input[b] = input[output]
        Gate::Mul(a, b, output) => {
          r1cs.add_constraints(
            vec![(Variable { index: *a, value: self.inputs[*a].clone() }, BigInt::from(1))],
            vec![(Variable { index: *b, value: self.inputs[*b].clone() }, BigInt::from(1))],
            vec![(Variable { index: *output, value: self.inputs[*output].clone() }, BigInt::from(1))],
            Operation::Mul
          );
        }
        // For a Hash gate, compute hash(input[a], input[b]) and enforce it equals output
        Gate::Hash(a, b, output) => {
          let hash_result = self.apply_hash(&self.inputs[*a], &self.inputs[*b]);
          r1cs.add_constraints(
            vec![(Variable { index: *a, value: self.inputs[*a].clone() }, BigInt::from(1))],
            vec![(Variable { index: *b, value: self.inputs[*b].clone() }, BigInt::from(1))],
            vec![(Variable { index: *output, value: hash_result.clone() }, BigInt::from(1))],
            Operation::Hash
          );

          println!("Hash-gate applied: hash({:?}, {:?}) = {:?}", self.inputs[*a], self.inputs[*b], hash_result);
        }
      }
    }

    // Checking for whether the witness would be satisfying the constraint that has been defined in the circuits defined in R1CS

    let is_valid = r1cs.is_satisfied(|a, b| {
      if let Some(ref hash_function) = self.hash_function {
        hash_function.hash(a, b) // the return value would be in terms of BigInt
      } else {
        panic!("Hash function not defined for this circuit");
      }
    });

    let mut file = std::fs::File::create(proof_file).expect("Unable to create proof file");
    file.write_all(&[is_valid as u8]).expect("Unable to write proof data"); // writing whether the proof is valid as a single byte (1 for valid, 0 for invalid one)
    println!("Proof-generation completed. Proof is valid: {}", is_valid);
  }

  pub fn verify_proof(&self, proof_file: &str) -> bool {
    let proof_data = std::fs::read(proof_file).expect("Unable to read proof file");
    if proof_data.len() != 1 {
      panic!("Invalid proof data format");
    }
    proof_data[0] == 1 // returns true if the proof is valid (1), false otherwise (0)
  }
}