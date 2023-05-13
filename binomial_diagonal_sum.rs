use num::bigint::BigInt;

fn f(n: BigInt) -> BigInt {
    if n < BigInt::from(2) {
        n
    } else {
        n.clone() * f(n - BigInt::from(1))
    }
}

fn c(x: BigInt, y: BigInt) -> BigInt {
    if x == y || y == BigInt::from(0) {
        return BigInt::from(1);
    }
    if y > x {
        BigInt::from(0)
    } else {
        f(x.clone()) / (f(y.clone()) * f(x - y))
    }
}

fn diagonal(n: u32, p: u32) -> u64 {
    (0..=n)
        .fold(BigInt::from(0), |a, i| {
            a + c(BigInt::from(i), BigInt::from(p))
        })
        .to_string()
        .parse::<u64>()
        .unwrap()
}
