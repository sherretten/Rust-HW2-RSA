use toy_rsa_lib::*;
use std::convert::{TryFrom, TryInto};
pub const EXP: u64 = 65_537;

pub fn genkey() -> (u32, u32) {
    let mut p: u32 = 0;
    let mut q: u32 = 0;
    let mut a: u64 = 0;

    loop {
        p = rsa_prime();
        q = rsa_prime();
        a = lambda(p, q) as u64;
        if EXP < a && gcd(EXP, a) == 1 {
            break;
        }
    }
    (p, q)
}

pub fn lambda(p: u32, q: u32) -> u64 {
    let mut a: u64 = lcm((p - 1).into(), (q - 1).into());
    return a;
}

pub fn encrypt(key: u64, msg: u32) -> u64 {
    modexp(msg as u64, EXP, key)
}

pub fn decrypt(key: (u32, u32), msg: u64) -> u32 {
    let d: u64 = modinverse(lambda(key.0, key.0).into(), EXP); // inverse of lambda p,q mod E

    modexp(msg, d, (key.0 * key.1).into()).try_into().unwrap()
}

fn main() {
    // let p: u32 = 0xed23e6cd;

    // let q: u32 = 0xf050a04d;

    // let publicKey = genkey();

    // let Message = 12345;
}
