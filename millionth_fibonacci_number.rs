use num::bigint::*;

fn fib(N: i32) -> BigInt {
    let mut a = 1.to_bigint().unwrap();
    let mut b = 0.to_bigint().unwrap();
    let mut p = 0.to_bigint().unwrap();
    let mut q = 1.to_bigint().unwrap();
    let sign = if N < 0 && (N.abs() & 1 == 0) { -1 } else { 1 };
    let mut n = N.abs();
    loop {
        if n == 0 {
            return b * sign;
        } else if n & 1 == 0 {
            (p, q) = (
                p.clone() * p.clone() + q.clone() * q.clone(),
                q.clone() * q.clone() + 2 * p.clone() * q.clone(),
            );
            n /= 2;
        } else {
            (a, b) = (
                b.clone() * q.clone() + a.clone() * q.clone() + a.clone() * p.clone(),
                b * p.clone() + a.clone() * q.clone(),
            );
            n -= 1;
        }
    }
}
