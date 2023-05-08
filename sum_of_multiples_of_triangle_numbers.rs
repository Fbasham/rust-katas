use num::bigint::BigUint;

fn gcd(a: BigUint, b: BigUint) -> BigUint {
    if b == BigUint::from(0 as u8) {
        a
    } else {
        gcd(b.clone(), a % b)
    }
}

fn lcm(a: BigUint, b: BigUint) -> BigUint {
    a.clone() * b.clone() / gcd(a, b)
}

fn sum_mult_triangnum(n: u32, m: u32) -> BigUint {
    let k = (1..=n)
        .map(|e| BigUint::from(e * (e + 1) / 2))
        .fold(BigUint::from(1 as u8), |a, c| lcm(a, c));
    (1..=m).map(|e| e * k.clone()).sum()
}
