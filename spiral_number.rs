use num::bigint::BigUint;

fn spiral_sum(n: BigUint) -> BigUint {
    let one = "1".parse::<BigUint>().unwrap();
    let two = "2".parse::<BigUint>().unwrap();
    (n + &one).pow(2) / two - one
}
