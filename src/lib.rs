//! A high-performance engine for working with factorials symbolically.
//!
//! Rather than computing `n!` directly (which overflows for even
//! moderately sized `n`), this crate uses **Legendre's Formula** to derive
//! the prime factorization of `n!` — a *symbolic* representation such as
//! `2^47 × 3^22 × 5^12 × ...` — without ever materializing the enormous
//! integer value itself. This makes the engine fast and immune to
//! overflow.
//!
//! The crate also provides a [`reverse_factorial`] function to invert the
//! process: given a value, determine which `n` (if any) satisfies `n! ==
//! value`.
#![deny(missing_docs)]

use num_bigint::BigUint;
use num_traits::One;
use std::collections::BTreeMap;
use std::fmt;
use std::ops::Mul;

/// Errors that can occur while working with factorials.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FactorialError {
    /// The given value is not the factorial of any integer.
    NotAFactorial(u128),
    /// The search for `n` overflowed before a match could be found.
    Overflow,
}

impl fmt::Display for FactorialError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FactorialError::NotAFactorial(v) => {
                write!(f, "{v} is not the factorial of any integer")
            }
            FactorialError::Overflow => {
                write!(f, "value is too large to be represented as a factorial")
            }
        }
    }
}

impl std::error::Error for FactorialError {}

/// The prime factorization of `n!`, expressed as `{prime: exponent}` pairs.
///
/// This is the "symbolic" form of a factorial: it fully determines the
/// value of `n!` without requiring the (potentially astronomically large)
/// integer to ever be computed.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SymbolicFactorial {
    factors: BTreeMap<u64, u64>,
}

impl SymbolicFactorial {
    /// Returns the underlying `{prime: exponent}` map.
    pub fn factors(&self) -> &BTreeMap<u64, u64> {
        &self.factors
    }

    /// Returns the exponent of `prime` in this factorization, or `0` if
    /// `prime` does not divide `n!`.
    pub fn exponent_of(&self, prime: u64) -> u64 {
        self.factors.get(&prime).copied().unwrap_or(0)
    }

    /// Combines two symbolic factorials as if multiplying the underlying
    /// factorials together, by summing exponents prime-by-prime.
    pub fn multiply(&self, rhs: &Self) -> Self {
        let mut factors = self.factors.clone();
        for (&p, &e) in &rhs.factors {
            *factors.entry(p).or_insert(0) += e;
        }
        Self { factors }
    }

    /// Divides this symbolic factorial by `rhs`, by subtracting exponents
    /// prime-by-prime.
    ///
    /// Returns `None` if any prime's exponent in `rhs` exceeds its exponent
    /// here, i.e. if the division would not be exact.
    pub fn checked_divide(&self, rhs: &Self) -> Option<Self> {
        let mut factors = self.factors.clone();
        for (&p, &e) in &rhs.factors {
            let cur = factors.get(&p).copied().unwrap_or(0);
            if cur < e {
                return None;
            }
            let next = cur - e;
            if next == 0 {
                factors.remove(&p);
            } else {
                factors.insert(p, next);
            }
        }
        Some(Self { factors })
    }

    /// Raises this symbolic factorial to the power `k`, by multiplying
    /// every exponent by `k`.
    pub fn pow(&self, k: u32) -> Self {
        let factors = self
            .factors
            .iter()
            .map(|(&p, &e)| (p, e * k as u64))
            .collect();
        Self { factors }
    }

    /// Reconstructs the exact integer value of this factorization as a
    /// [`BigUint`], by multiplying out `prime^exponent` for every prime.
    pub fn to_biguint(&self) -> BigUint {
        self.factors
            .iter()
            .fold(BigUint::one(), |acc, (&p, &e)| {
                acc * BigUint::from(p).pow(e as u32)
            })
    }
}

impl Mul for &SymbolicFactorial {
    type Output = SymbolicFactorial;

    fn mul(self, rhs: &SymbolicFactorial) -> SymbolicFactorial {
        self.multiply(rhs)
    }
}

impl fmt::Display for SymbolicFactorial {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.factors.is_empty() {
            return write!(f, "1");
        }
        let terms: Vec<String> = self
            .factors
            .iter()
            .map(|(p, e)| {
                if *e == 1 {
                    p.to_string()
                } else {
                    format!("{p}^{e}")
                }
            })
            .collect();
        write!(f, "{}", terms.join(" \u{d7} "))
    }
}

/// An engine for computing the symbolic (prime-factorized) form of `n!`.
pub struct FactorialEngine {
    primes_cache: Vec<u64>,
}

impl Default for FactorialEngine {
    fn default() -> Self {
        Self::new(None)
    }
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

    /// Returns the symbolic (prime-factorized) form of `n!`.
    ///
    /// This is the primary public method of the engine: it computes the
    /// factorization of `n!` directly via Legendre's Formula, without ever
    /// computing `n!` itself.
    pub fn symbolic_factorial(&mut self, n: u64) -> SymbolicFactorial {
        if n < 2 {
            return SymbolicFactorial::default(); // 0! and 1! have no prime factors.
        }

        // Ensure we have all necessary primes cached.
        if self.primes_cache.last().is_none_or(|&max_p| max_p < n) {
            self.sieve_primes(n);
        }

        let mut factors = BTreeMap::new();
        for &p in self.primes_cache.iter().take_while(|&&pr| pr <= n) {
            let exponent = self.calculate_exponent(n, p);
            if exponent > 0 {
                factors.insert(p, exponent);
            }
        }

        SymbolicFactorial { factors }
    }

    /// Computes the exact value of `n!` as a [`BigUint`].
    ///
    /// This first derives the symbolic (prime-factorized) form via
    /// [`FactorialEngine::symbolic_factorial`] and then multiplies it out,
    /// so it remains overflow-free right up until the final, unavoidably
    /// large, result.
    pub fn factorial_biguint(&mut self, n: u64) -> BigUint {
        self.symbolic_factorial(n).to_biguint()
    }

    /// Computes the binomial coefficient `C(n, k)` = `n! / (k! * (n-k)!)`
    /// as a [`BigUint`], entirely through symbolic factorial arithmetic.
    ///
    /// Returns `None` if `k > n`.
    pub fn binomial(&mut self, n: u64, k: u64) -> Option<BigUint> {
        if k > n {
            return None;
        }
        let numerator = self.symbolic_factorial(n);
        let denominator = self
            .symbolic_factorial(k)
            .multiply(&self.symbolic_factorial(n - k));
        numerator.checked_divide(&denominator).map(|result| result.to_biguint())
    }

    /// Returns the prime factorization of `n!` as a `HashMap` of `{prime: exponent}`.
    ///
    /// Kept for backwards compatibility; prefer [`FactorialEngine::symbolic_factorial`],
    /// which returns a richer, displayable [`SymbolicFactorial`] value.
    #[deprecated(since = "0.3.0", note = "use `symbolic_factorial` instead")]
    pub fn get_factorial_factorization(&mut self, n: u64) -> std::collections::HashMap<u64, u64> {
        self.symbolic_factorial(n).factors.into_iter().collect()
    }
}

/// Finds the integer `n` such that `n! == value`, if one exists.
///
/// This is the inverse of the factorial function: rather than computing
/// `n!` from `n`, it recovers `n` from a candidate factorial value.
///
/// # Errors
/// Returns [`FactorialError::NotAFactorial`] if `value` is not the
/// factorial of any integer, or [`FactorialError::Overflow`] if `value`
/// exceeds what can be represented while searching.
///
/// # Examples
/// ```
/// use factorial_engine::reverse_factorial;
///
/// assert_eq!(reverse_factorial(120), Ok(5)); // 5! == 120
/// assert!(reverse_factorial(121).is_err());
/// ```
pub fn reverse_factorial(value: u128) -> Result<u64, FactorialError> {
    let mut n: u64 = 0;
    let mut acc: u128 = 1;

    while acc < value {
        n = n.checked_add(1).ok_or(FactorialError::Overflow)?;
        acc = acc.checked_mul(n as u128).ok_or(FactorialError::Overflow)?;
    }

    if acc == value {
        Ok(n)
    } else {
        Err(FactorialError::NotAFactorial(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn symbolic_factorial_of_fifty_matches_known_exponents() {
        let mut engine = FactorialEngine::new(Some(100));
        let factors = engine.symbolic_factorial(50);
        assert_eq!(factors.exponent_of(2), 47);
        assert_eq!(factors.exponent_of(3), 22);
    }

    #[test]
    fn symbolic_factorial_of_zero_and_one_is_empty() {
        let mut engine = FactorialEngine::new(None);
        assert!(engine.symbolic_factorial(0).factors().is_empty());
        assert!(engine.symbolic_factorial(1).factors().is_empty());
    }

    #[test]
    fn symbolic_factorial_display_format() {
        let mut engine = FactorialEngine::new(None);
        let factors = engine.symbolic_factorial(6); // 6! = 720 = 2^4 * 3^2 * 5
        assert_eq!(factors.to_string(), "2^4 \u{d7} 3^2 \u{d7} 5");
    }

    #[test]
    fn factorial_biguint_matches_expected_value() {
        let mut engine = FactorialEngine::new(None);
        assert_eq!(engine.factorial_biguint(10), BigUint::from(3628800u64));
    }

    #[test]
    fn multiply_and_checked_divide_round_trip() {
        let mut engine = FactorialEngine::new(None);
        let five = engine.symbolic_factorial(5);
        let three = engine.symbolic_factorial(3);
        let product = five.multiply(&three);
        assert_eq!(product.to_biguint(), BigUint::from(120u64 * 6u64));
        assert_eq!(product.checked_divide(&three).unwrap(), five);
        assert!(three.checked_divide(&five).is_none()); // 5! has prime factors 3! lacks
    }

    #[test]
    fn pow_multiplies_exponents() {
        let mut engine = FactorialEngine::new(None);
        let three = engine.symbolic_factorial(3); // 3! = 6 = 2 * 3
        let squared = three.pow(2);
        assert_eq!(squared.to_biguint(), BigUint::from(36u64));
    }

    #[test]
    fn binomial_matches_known_values() {
        let mut engine = FactorialEngine::new(None);
        assert_eq!(engine.binomial(5, 2), Some(BigUint::from(10u64)));
        assert_eq!(engine.binomial(10, 0), Some(BigUint::from(1u64)));
        assert_eq!(engine.binomial(3, 5), None);
    }

    #[test]
    fn reverse_factorial_finds_known_values() {
        assert_eq!(reverse_factorial(1), Ok(0)); // 0! == 1
        assert_eq!(reverse_factorial(120), Ok(5));
        assert_eq!(reverse_factorial(3628800), Ok(10));
    }

    #[test]
    fn reverse_factorial_rejects_non_factorials() {
        assert_eq!(
            reverse_factorial(121),
            Err(FactorialError::NotAFactorial(121))
        );
        assert_eq!(reverse_factorial(0), Err(FactorialError::NotAFactorial(0)));
    }
}
