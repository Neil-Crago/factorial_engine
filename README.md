# Factorial Engine

[![Crates.io](https://img.shields.io/crates/v/factorial_engine.svg?style=flat-square)](https://crates.io/crates/factorial_engine)
[![Docs.rs](https://img.shields.io/docsrs/factorial_engine?style=flat-square)](https://docs.rs/factorial_engine)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue?style=flat-square)](https://opensource.org/licenses/MIT)
[![Rust](https://github.com/Neil-Crago/factorial_engine/actions/workflows/rust.yml/badge.svg)](https://github.com/Neil-Crago/factorial_engine/actions/workflows/rust.yml)

A high-performance, zero-error Rust crate for computing the prime factorization of factorials (`n!`).

This engine is designed as a robust, backend computational tool. It uses **Legendre's Formula** to calculate prime exponents directly, completely avoiding the need to compute or store the immense values of `n!` itself. This ensures exceptional performance and prevents any possibility of integer overflow, even for very large `n`.

## Features

- **High Performance:** Employs Legendre's Formula for direct calculation of prime exponents.
- **Zero Error:** Avoids large number arithmetic entirely, making it robust and free from overflow errors.
- **Efficient Prime Generation:** Includes an optimized Sieve of Eratosthenes for on-demand prime generation and caching.
- **Symbolic Factorials:** [`SymbolicFactorial`] represents `n!` as a displayable prime factorization (e.g. `2^47 × 3^22 × 5^12 × ...`).
- **Symbolic Arithmetic:** `multiply`, `checked_divide`, and `pow` combine symbolic factorials (e.g. for binomial coefficients) without ever computing the underlying integers.
- **BigUint Support:** `to_biguint`, `FactorialEngine::factorial_biguint`, and `FactorialEngine::binomial` materialize exact, arbitrary-precision results via [`num-bigint`](https://crates.io/crates/num-bigint) only when you actually need the number.
- **Reverse Factorial:** [`reverse_factorial`] recovers `n` from a candidate factorial value, e.g. `reverse_factorial(120) == Ok(5)`.
- **Clean API:** Provides a simple and clear interface for getting the full symbolic factorization of `n!`.

## Usage

Add this crate to your `Cargo.toml`:

```toml
[dependencies]
factorial_engine = "0.4" # Or the latest version
```

## Example

```Rust
use factorial_engine::{reverse_factorial, FactorialEngine};

fn main() {
    // Initialize the engine. Can optionally pre-sieve primes.
    let mut engine = FactorialEngine::new(Some(100));

    let n = 50;
    let factors = engine.symbolic_factorial(n);

    // Displays as "2^47 × 3^22 × 5^12 × ...".
    println!("Symbolic factorization of {}!: {}", n, factors);

    // Example: The exponent of 2 in 50! is 47.
    assert_eq!(factors.exponent_of(2), 47);

    // Reverse factorial: recover n such that n! == value.
    assert_eq!(reverse_factorial(120), Ok(5));
    assert!(reverse_factorial(121).is_err());

    // Exact, arbitrary-precision values via BigUint.
    println!("50! = {}", engine.factorial_biguint(50));

    // Binomial coefficients computed entirely through symbolic arithmetic.
    assert_eq!(engine.binomial(5, 2), Some(10u32.into()));
}
```

## Purpose

This crate serves as a foundational block for applications in number theory, combinatorics, and computational mathematics. It is designed to be a reliable, "black-box" dependency that provides factorial factorization data with maximum efficiency and correctness.

## Author

Neil Crago

## Related Crates

This crate is part of a collection of crates by the same author:
These include:-

* MOMA
* MOMA_simulation_engine
* Fractal_Algebra
* tma_engine
* fa_slow_ai
