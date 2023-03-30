use num::bigint::BigUint;
use num::Zero;

fn highest_bi_prime_factor(p1: u32, p2: u32, n: BigUint) -> (BigUint, u32, u32) {
    let mut m = BigUint::zero();
    let mut x = 0;
    let mut y = 0;
    for i in 1..100 {
        for j in 1..100 {
            let t = BigUint::from(p1).pow(i) * BigUint::from(p2).pow(j);
            if t <= n && t > m {
                (m, x, y) = (t, i, j);
            }
        }
    }
    (m, x, y)
}
