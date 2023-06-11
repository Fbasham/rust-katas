use num::bigint::BigUint;
use std::str::FromStr;

fn count_pal(n: u32) -> (BigUint, BigUint) {
    let k = ((n as f64 / 2.0).ceil() as usize).saturating_sub(1);
    let x = format!("9{}", "0".repeat(k));
    let y = if n == 1 {
        "9".to_string()
    } else {
        format!(
            "1{}{}8",
            if n % 2 == 1 { "0" } else { "" },
            "9".repeat(k.saturating_sub(if n % 2 == 1 { 1 } else { 0 }))
        )
    };
    (
        BigUint::from_str(x.as_str()).unwrap(),
        BigUint::from_str(y.as_str()).unwrap(),
    )
}
