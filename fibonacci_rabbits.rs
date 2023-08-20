use num::bigint::BigUint;

fn fib_rabbits(n: u8, x: u8) -> BigUint {
    let mut a = "1".parse::<BigUint>().unwrap();
    let mut b = "0".parse::<BigUint>().unwrap();
    for _ in 0..n {
        (a, b) = (b.clone() * x, a.clone() + b);
    }
    b
}
