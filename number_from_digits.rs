use itertools::Itertools;
use num::bigint::BigUint;

fn f(n: BigUint) -> BigUint {
    if n < BigUint::from(2 as u32) {
        n
    } else {
        n.clone() * f(n.clone() - BigUint::from(1 as u32))
    }
}

fn proc_arr(a: &[char]) -> (BigUint, BigUint, BigUint) {
    let v = f(BigUint::from(a.len()))
        / a.iter()
            .counts()
            .values()
            .map(|e| f(BigUint::from(*e)))
            .product::<BigUint>();
    (
        v,
        a.iter().sorted().join("").parse::<BigUint>().unwrap(),
        a.iter().sorted().rev().join("").parse::<BigUint>().unwrap(),
    )
}
