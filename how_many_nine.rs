use num::bigint::BigInt;
use num::bigint::ToBigInt;
use num::traits::Zero;

fn nines(n: BigInt) -> BigInt {
    let mut s = n.to_string();
    if s.contains("9") {
        let i = s.chars().position(|e| e == '9').unwrap();
        s = format!("{}{}", &s[..i], "8".repeat(s.len() - i));
    }
    n - s.chars().enumerate().fold(BigInt::zero(), |a, (i, c)| {
        a + (c as u32 - 48) * 9.to_bigint().unwrap().pow((s.len() - i - 1) as u32)
    })
}
