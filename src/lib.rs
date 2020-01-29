use toy_rsa_lib::*;
use std::convert::{TryInto};
pub const EXP: u64 = 65_537;

pub fn genkey() -> (u32, u32) {
    // let mut p: u32 = 0;
    // let mut q: u32 = 0;
    // let mut a: u64 = 0;

    loop {
        let p: u32 = rsa_prime();
        let q: u32 = rsa_prime();
        let a: u64 = lambda(p, q);
        if EXP < a && gcd(EXP, a) == 1 {
          return (p,q);
        }
    }
}

pub fn lambda(p: u32, q: u32) -> u64 {
    lcm((p - 1).into(), (q - 1).into())
}

pub fn encrypt(key: u64, msg: u32) -> u64 {
    modexp(msg as u64, EXP, key)
}

pub fn decrypt(key: (u32, u32), msg: u64) -> u32 {
    let d: u64 = modinverse(EXP, lambda(key.0, key.1).into()); // inverse of lambda p,q mod E
    let n: u64 = (key.0 as u64) * (key.1 as u64);
    modexp(msg, d, n).try_into().expect("Overflow")
}


