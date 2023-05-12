use num::bigint::BigInt;

fn f(n: BigInt) -> BigInt {
    if n < BigInt::from(2) {
        BigInt::from(1)
    } else {
        n.clone() * f(n - BigInt::from(1))
    }
}

fn c(x: u64, y: u64) -> BigInt {
    f(BigInt::from(x)) / (f(BigInt::from(y)) * f(BigInt::from(x - y)))
}

fn check_choose(m: u64, n: u64) -> i64 {
    for i in 1..n {
        if c(n, i) == BigInt::from(m) {
            return i as i64;
        }
    }
    -1
}
