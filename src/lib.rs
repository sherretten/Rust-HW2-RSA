use toy_rsa_lib::*;

pub const EXP: u64 = 65_537;

pub fn genkey() -> (u32, u32) {
    let mut p: u32 = 0;
    let mut q: u32 = 0;
    let mut a: u32 = 0;

    loop {
        p = rsa_prime();
        q = rsa_prime();
        a = lambda(p, q);
        if EXP < a.try_into().unwrap() && gcd(EXP, a) == 1 {
            break;
        }
    }

    (p, q)
}

pub fn lambda(p: u32, q: u32) {
    lcm(p - 1, q - 1)
}

pub fn encrypt(key: u64, msg: u32) -> u64 {
    modexp(msg as u64, EXP, key)
}

pub fn decrypt(key: (u32, u32), msg: u64) -> u32 {
    let d: u64 = modinverse(lambda(key.0, key.0), EXP); // inverse of lambda p,q mod E

    modexp(msg, d, key.0 * key.1)
}

fn main() {
    let p = 0xed23e6cd;

    let q = 0xf050a04d;

    let publicKey = genkey();

    let Message = 12345;
}
