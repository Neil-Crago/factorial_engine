[![Crates.io](https://img.shields.io/crates/v/factorial_engine.svg?style=flat-square)](https://crates.io/crates/factorial_engine)
[![Docs.rs](https://img.shields.io/docsrs/factorial_engine?style=flat-square)](https://docs.rs/factorial_engine)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue?style=flat-square)](https://opensource.org/licenses/MIT)
[![Rust](https://github.com/Neil-Crago/factorial_engine/actions/workflows/rust.yml/badge.svg)](https://github.com/Neil-Crago/factorial_engine/actions/workflows/rust.yml)

# Factorial Engine

A high-performance, zero-error Rust crate for computing the prime factorization of factorials (`n!`).

This engine is designed as a robust, backend computational tool. It uses **Legendre's Formula** to calculate prime exponents directly, completely avoiding the need to compute or store the immense values of `n!` itself. This ensures exceptional performance and prevents any possibility of integer overflow, even for very large `n`.

## Features

- **High Performance:** Employs Legendre's Formula for direct calculation of prime exponents.
- **Zero Error:** Avoids large number arithmetic entirely, making it robust and free from overflow errors.
- **Efficient Prime Generation:** Includes an optimized Sieve of Eratosthenes for on-demand prime generation and caching.
- **Clean API:** Provides a simple and clear interface for getting the full prime factorization of `n!`.

## Usage

Add this crate to your `Cargo.toml`:
```toml
[dependencies]
factorial_engine = "0.1.8" # Or the latest version
```

## Example
```Rust
use factorial_engine::FactorialEngine;
use std::collections::HashMap;

fn main() {
    // Initialize the engine. Can optionally pre-sieve primes.
    let mut engine = FactorialEngine::new(Some(100));

    let n = 50;
    let factors: HashMap<u64, u64> = engine.get_factorial_factorization(n);

    // The result is a map of {prime: exponent}.
    println!("Prime factorization of {}!: {:?}", n, factors);

    // Example: The exponent of 2 in 50! is 47.
    assert_eq!(*factors.get(&2).unwrap(), 47);
}
```

## Purpose
This crate serves as a foundational block for applications in number theory, combinatorics, and computational mathematics. It is designed to be a reliable, "black-box" dependency that provides factorial factorization data with maximum efficiency and correctness.

## Author
Neil Crago — experimental mathematician

## Related Crates
This crate is part of a collection of crates by the same author:
These include:-
  * MOMA
  * MOMA_simulation_engine
  * Fractal_Algebra
  * tma_engine
  * fa_slow_ai