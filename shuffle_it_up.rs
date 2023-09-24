use num::bigint::*;
use num::ToPrimitive;

fn f(n: BigInt) -> BigInt {
    if n == BigInt::from(0) {
        BigInt::from(1)
    } else {
        n.clone() * (f(n.clone() - 1)) + BigInt::from(-1).pow(n.to_u32().unwrap())
    }
}

fn all_permuted(n: u32) -> BigUint {
    f(n.into()).to_biguint().unwrap()
}
