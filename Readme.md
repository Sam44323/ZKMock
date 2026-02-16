# ZKMock

_ZKMock_ is a minimal Rust project intended to explore and experiment with the core ideas behind Zero-Knowledge Proof systems through a simplified, mock implementation.
The goal of this repository is conceptual clarity, not cryptographic security or production readiness.

---

### This project is suitable for:

- Learning how ZKP systems are structured

- Understanding prover/verifier flows

- Prototyping the ideas before moving to real-systems such as PLONK, Groth16, or Halo2

---

### Scope

#### In Scope

- Basic abstractions for a prover and verifier

- Clear in-separation of proof generation and verification

- Deterministic, inspectable logic for learning and debugging

- Rust-first design using explicit types

### Out of Scope

- Cryptographic based security

- Real elliptic curves or finite field arithmetic

- Performance opted optimizations

- Compatibility with production ZKP libraries

> Note: This repository must not be used for real cryptographic-applications.

### Files

`circuit.rs` - The high-level logic for the zkp for defining the circuit which then would be used to convert to r1cs constraint system.

`r1cs.rs` - The logic for converting the circuit to r1cs constraint system, which is a common representation for zkps.

`hash_function.rs` - A simple hash function implementation to be used in the circuit, not cryptographically secure.

`merkle.rs` - A simple Merkle tree implementation to demonstrate how Merkle proofs can be used in zkps.

shoutout to @luisshr for this: https://github.com/luishsr/zk-mini

---
