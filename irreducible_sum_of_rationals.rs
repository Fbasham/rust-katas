fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: i64, b: i64) -> i64 {
    a * b / gcd(a, b)
}

fn sum_fracts(a: Vec<(i64, i64)>) -> Option<(i64, i64)> {
    if a.is_empty() {
        return None;
    }
    let d = a.iter().fold(1, |a, c| lcm(a, c.1));
    let t = a
        .into_iter()
        .reduce(|a, c| (a.0 * d / a.1 + c.0 * d / c.1, d))
        .unwrap();
    Some((t.0 / gcd(t.0, t.1), t.1 / gcd(t.0, t.1)))
}
