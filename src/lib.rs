pub mod elgamel;

use rand::{thread_rng, Rng};

/// Determine `n` is prime number.
pub fn is_prime(n: u64) -> bool {
    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 1;
    }
    true
}

/// Generate specified bits prime number.
pub fn generate_prime(bits: u8) -> u64 {
    if bits == 0 {
        panic!("Number of bits should be non-zero");
    }

    let mut rng = thread_rng();
    let lower_bound = 1 << (bits - 1);
    let upper_bound = 1 << bits;
    loop {
        let n = rng.gen_range(lower_bound..upper_bound);
        if is_prime(n) {
            return n;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prime() {
        assert!(is_prime(1e9 as u64 + 7));
    }

    #[test]
    fn get_prime() {
        let n = generate_prime(16);
        assert!(is_prime(n));
    }
}
