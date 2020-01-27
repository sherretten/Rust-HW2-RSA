use toy_rsa_lib::*;
pub const EXP = 65_537;

pub fn genkey() -> (u32, u32){
    loop {
        let p = rsa_prime();
        let q = rsa_prime();
        let a: u32 = lambda(p, q);
        if EXP < a.try_into().unwrap() && gcd(EXP, a) == 1 { return (p, q); }
    }
    
}

pub fn lambda ( p: u32, q:u32){
  lcm(p-1, q-1)
}
pub fn encrypt(key: u64, msg: u32) -> u64 {
  modexp(msg, EXP, key)
}

pub fn decrypt(key: (u32, u32), msg: u64) -> u32 {
  let d = 1; // inverse of lambda p,q mod E
  modexp(msg, d, (key.0)* (key.1))
}

fn main() {
  let p = 0xed23e6cd; 
  let q = 0xf050a04d;
  let publicKey = genkey();
  let Message = 12345;

}