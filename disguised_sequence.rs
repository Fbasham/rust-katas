use num::bigint::BigInt;

fn fcn(n: i32) -> i64 {
    let mut a = BigInt::from(1);
    let mut b = BigInt::from(2);
    for _ in 0..n {
        (a, b) = (
            b.clone(),
            BigInt::from(6) * a.clone() * b.clone() / (BigInt::from(5) * a.clone() - b.clone()),
        )
    }
    a.to_string().parse().unwrap()
}
