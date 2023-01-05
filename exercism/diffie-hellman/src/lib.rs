use rand::Rng;
pub fn private_key(p: u64) -> u64 {
    rand::thread_rng().gen_range(2..p)
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    modular_pow(g, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    modular_pow(b_pub, a, p)
}

// https://en.wikipedia.org/wiki/Modular_exponentiation#Pseudocode
// function modular_pow(base, exponent, modulus) is
//     if modulus = 1 then
//         return 0
//     Assert :: (modulus - 1) * (modulus - 1) does not overflow base
//     result := 1
//     base := base mod modulus
//     while exponent > 0 do
//         if (exponent mod 2 == 1) then
//             result := (result * base) mod modulus
//         exponent := exponent >> 1
//         base := (base * base) mod modulus
//     return result
fn modular_pow(base: u64, exponent: u64, modulus: u64) -> u64 {
    let mut exponent = exponent;
    let mut base = base as u128;
    let mut result = 1;
    let modulus = modulus as u128;

    if modulus <= 1 {
        return 0;
    }

    while exponent > 0 {
        if exponent % 2 == 1 {
            result = (result * base) % modulus;
        }
        base = (base * base) % modulus;
        exponent = exponent / 2;
    }
    result as u64
}
