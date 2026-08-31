use factorial_engine::{reverse_factorial, FactorialEngine};

fn main() {
    println!("Initializing FactorialEngine...");
    let mut engine = FactorialEngine::new(Some(100)); // Pre-calculate primes up to 100

    let n = 50;
    println!("\nCalculating symbolic factorization of {}!...", n);

    let start_time = std::time::Instant::now();
    let factors = engine.symbolic_factorial(n);
    let duration = start_time.elapsed();

    println!("Calculation complete in {:?}.", duration);
    println!("Result for {}!: {}", n, factors);

    // Example: Exponent of 2 in 50! should be 47.
    // 50/2=25, 50/4=12, 50/8=6, 50/16=3, 50/32=1 -> 25+12+6+3+1 = 47
    println!("Exponent of 2 is: {}", factors.exponent_of(2));

    // Reverse factorial: recover n from n!.
    println!("\nreverse_factorial(120) = {:?}", reverse_factorial(120));
    println!("reverse_factorial(121) = {:?}", reverse_factorial(121));
}

