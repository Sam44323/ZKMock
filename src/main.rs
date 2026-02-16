mod circuit;
mod r1cs;
mod hash_functions;
mod merkle;

use num_bigint::{BigInt, ToBigInt};
use circuit::Circuit;
use hash_functions::{SimpleAddHash, CustomHash};

/// A simple addition proof using the Circuit and a basic addition hash 
fn addition_proof() {
    let mut circuit = Circuit::new(None);

    let input1 = circuit.add_input(10.to_bigint().unwrap());
    let input2 = circuit.add_input(20.to_bigint().unwrap());

    let output_index = circuit.add_input(30.to_bigint().unwrap()); // This will be the expected output of the addition
    circuit.add_gate(circuit::Gate::Add(input1, input2, output_index));
    circuit.add_output(30.to_bigint().unwrap());

    // generating the verifying the addition-proof
    println!("Generating addition-proof...");
    circuit.generate_proof("addition_proof.bin");
    let is_valid = circuit.verify_proof("addition_proof.bin");
    println!("Addition-proof is valid: {}", is_valid);
}

fn multiplication_proof() {
    let mut circuit = Circuit::new(None);

    let input1 = circuit.add_input(5.to_bigint().unwrap());
    let input2 = circuit.add_input(4.to_bigint().unwrap());

    let output_index = circuit.add_input(20.to_bigint().unwrap()); // This will be the expected output of the multiplication
    circuit.add_gate(circuit::Gate::Mul(input1, input2, output_index));
    circuit.add_output(20.to_bigint().unwrap());

    // generating the verifying the multiplication-proof
    println!("Generating multiplication-proof...");
    circuit.generate_proof("multiplication_proof.bin");
    let is_valid = circuit.verify_proof("multiplication_proof.bin");
    println!("Multiplication-proof is valid: {}", is_valid);
}

fn main() {
    addition_proof();
    multiplication_proof();
}
