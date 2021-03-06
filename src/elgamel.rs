use crate::{generate_prime, is_prime, mod_pow};
use rand::{thread_rng, Rng};

pub fn generate_key(bits: u8) -> ((u64, u64, u64), u64) {
    let (p, q) = loop {
        let q = generate_prime(bits);
        let p = 2 * q + 1;
        if is_prime(p) {
            break (p, q);
        }
    };

    let mut rng = thread_rng();
    // Generate primitive element `g`
    let g = loop {
        let g: u64 = rng.gen_range(3..p);
        if mod_pow(g, 2, p) == 1 {
            continue;
        } else if mod_pow(g, q, p) == 1 {
            continue;
        }
        break g;
    };

    // Secret key
    let x = rng.gen_range(2..(p - 1));
    // Public key
    let y = mod_pow(g, x, p);
    ((p, g, y), x)
}

pub fn encrypt(m: u64, public_key: (u64, u64, u64)) -> (u64, u64) {
    let mut rng = thread_rng();
    let (p, g, y) = public_key;
    let r = rng.gen_range(0..(p - 1));
    let c1 = mod_pow(g, r, p);
    let c2 = (m * mod_pow(y, r, p)) % p;
    (c1, c2)
}

pub fn decrypt(ciphered: (u64, u64), public_key: (u64, u64, u64), secret_key: u64) -> u64 {
    let (p, _, _) = public_key;
    let (c1, c2) = ciphered;
    (c2 * mod_pow(c1, p - 1 - secret_key, p)) % p
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_elgamel() {
        let (public_key, secret_key) = generate_key(20);
        let m = 314158;
        let c = encrypt(m, public_key);
        let d = decrypt(c, public_key, secret_key);
        assert_eq!(m, d);
    }
}
