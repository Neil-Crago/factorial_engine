use std::collections::HashMap;

/// A high-performance engine for computing the prime factorization of factorials.
/// It uses Legendre's Formula to avoid calculating large numbers, ensuring
/// speed and zero overflow errors.
pub struct FactorialEngine {
    primes_cache: Vec<u64>,
}

impl FactorialEngine {
    /// Creates a new engine. Can optionally pre-sieve primes up to a limit.
    pub fn new(sieve_up_to: Option<u64>) -> Self {
        let mut engine = FactorialEngine {
            primes_cache: Vec::new(),
        };
        if let Some(limit) = sieve_up_to {
            engine.sieve_primes(limit);
        }
        engine
    }

    /// Generates and caches primes up to a given limit using a Sieve of Eratosthenes.
    fn sieve_primes(&mut self, limit: u64) {
        if limit < 2 {
            return;
        }
        let mut is_prime = vec![true; (limit + 1) as usize];
        is_prime[0] = false;
        is_prime[1] = false;

        for p in 2..=(limit as f64).sqrt() as u64 {
            if is_prime[p as usize] {
                for i in (p * p..=limit).step_by(p as usize) {
                    is_prime[i as usize] = false;
                }
            }
        }
        self.primes_cache = is_prime
            .iter()
            .enumerate()
            .filter(|&(_, &is_p)| is_p)
            .map(|(p, _)| p as u64)
            .collect();
    }

    /// Calculates the exponent of a single prime `p` in the factorization of `n!`
    /// using Legendre's Formula.
    fn calculate_exponent(&self, n: u64, p: u64) -> u64 {
        let mut exponent = 0;
        let mut p_power = p;
        while p_power <= n {
            exponent += n / p_power;
            // Check for potential overflow before multiplying
            if p > u64::MAX / p_power {
                break;
            }
            p_power *= p;
        }
        exponent
    }

    /// Returns the prime factorization of n! as a HashMap of {prime: exponent}.
    /// This is the primary public method of the engine.
    pub fn get_factorial_factorization(&mut self, n: u64) -> HashMap<u64, u64> {
        if n < 2 {
            return HashMap::new(); // 0! and 1! have no prime factors.
        }

        // Ensure we have all necessary primes cached.
        if self.primes_cache.last().is_none_or(|&max_p| max_p < n) {
            self.sieve_primes(n);
        }

        let mut factorization = HashMap::new();

        for &p in self.primes_cache.iter().take_while(|&&pr| pr <= n) {
            let exponent = self.calculate_exponent(n, p);
            if exponent > 0 {
                factorization.insert(p, exponent);
            }
        }

        factorization
    }
}
