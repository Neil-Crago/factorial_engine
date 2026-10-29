/// Example usage of the factorial_engine crate.
/// Demonstrates deterministic primality testing using the is_prime_factorial function.
/// This example covers initializing the engine and checking the primality of numbers from 2 to 6.
/// The output will indicate whether each number in the range is prime according to the factorial-based test.
/// It uses the FactorialEngine to perform the checks.
use factorial_engine::FactorialEngine;

fn main() {
    let mut engine = FactorialEngine::new(None);
    for n in 2..=6 {
        println!("is_prime_factorial({}) = {}", n, FactorialEngine::is_prime_factorial(n, &mut engine));
    }
}
