extern crate num_bigint_dig as num_bigint;
extern crate num_traits;

use num_bigint::{BigInt, BigUint, ToBigInt, ToBigUint};
use num_traits::{One, Zero};
use std::fmt;
use std::result::Result;

const E: u32 = 65537;

struct PubKey {
    n: BigUint,
    e: BigUint,
}

struct PrivKey {
    n: BigUint,
    d: BigUint,
}

struct KeyPair {
    private_key: PrivKey,
    public_key: PubKey,
}

impl KeyPair {
    fn new(p: BigUint, q: BigUint) -> KeyPair {
        let n = &p * &q;
        let totient: BigUint = (p - BigUint::one()) * (q - BigUint::one());
        let d = multiplicative_inverse(BigUint::from(E), totient).unwrap();
        let pubKey = PubKey {
            n: n.clone(),
            e: BigUint::from(E),
        };
        let privKey = PrivKey { n: n, d: d };
        KeyPair {
            private_key: privKey,
            public_key: pubKey,
        }
    }
}

impl fmt::Display for KeyPair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "N: {}, E: {}, D: {}",
            self.public_key.n, self.public_key.e, self.private_key.d
        )
    }
}

fn multiplicative_inverse(n: BigUint, module: BigUint) -> Result<BigUint, String> {
    let mut r = vec![module.clone(), n];
    let mut p: Vec<BigInt> = vec![BigInt::zero(), BigInt::one()];
    while r[r.len() - 1] != Zero::zero() {
        let r0 = r[r.len() - 2].clone();
        let r1 = r[r.len() - 1].clone();
        let remainder = &r0 % &r1;
        let mul = r0 / r1;
        r.push(remainder);
        p.push(p[p.len() - 2].clone() - p[p.len() - 1].clone() * mul.to_bigint().unwrap());
    }
    if r[r.len() - 2] != BigUint::one() {
        return Err(String::from("No multiplicative possible"));
    }
    Ok(modulo(p[p.len() - 2].clone(), module.to_bigint().unwrap()))
}

fn encrypt(message: &BigUint, key: &PubKey) -> BigUint {
    message.modpow(&key.e, &key.n)
}

fn decrypt(cipher: &BigUint, key: &PrivKey) -> BigUint {
    cipher.modpow(&key.d, &key.n)
}

fn modulo(n: BigInt, m: BigInt) -> BigUint {
    let mut n = n % &m;
    while n < BigInt::zero() {
        n += &m;
    }
    n.to_biguint().unwrap()
}

pub fn run() {
    println!("--- RSA ---");
    let keypair = KeyPair::new(BigUint::from(419u32), BigUint::from(541u32));
    let c = encrypt(&BigUint::from(300u32), &keypair.public_key);
    let m = decrypt(&c, &keypair.private_key);
    println!("{}", &keypair);
    println!("{} {}", c, m);
}
