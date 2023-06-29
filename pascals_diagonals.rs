use num::bigint::BigUint;

fn f(n: u64) -> BigUint {
    (1..=n).fold("1".parse::<BigUint>().unwrap(), |x, y| {
        x * y.to_string().parse::<BigUint>().unwrap()
    })
}

fn c(x: u64, y: u64) -> BigUint {
    f(x) / (f(y) * f(x - y))
}

fn generate_diagonal(b: u8, l: usize) -> Vec<u64> {
    (0..l as u64)
        .map(|i| c(b as u64 + i, i).to_u64_digits()[0])
        .collect()
}
