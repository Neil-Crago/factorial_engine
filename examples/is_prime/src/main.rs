use factorial_engine::FactorialEngine;

fn main() {
    let mut engine = FactorialEngine::new(None);
    for n in 2..=6 {
        println!("is_prime_factorial({}) = {}", n, FactorialEngine::is_prime_factorial(n, &mut engine));
    }
}
