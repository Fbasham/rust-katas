use num::bigint::BigUint;
use num::bigint::ToBigUint;
use num::traits::*;

fn gcd(a: BigUint, b: BigUint) -> BigUint {
    if b > BigUint::zero() {
        gcd(b.clone(), a.clone() % b.clone())
    } else {
        a
    }
}

fn lcm(a: BigUint, b: BigUint) -> BigUint {
    a.clone() * b.clone() / gcd(a.clone(), b.clone())
}

fn candies_to_buy(n: u16) -> BigUint {
    (1..=n).fold(BigUint::one(), |a, c| lcm(a, c.to_biguint().unwrap()))
}
