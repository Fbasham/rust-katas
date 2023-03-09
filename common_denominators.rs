fn gcd(a: i64, b: i64) -> i64 {
    if b > 0 {
        gcd(b, a % b)
    } else {
        a
    }
}

fn lcm(a: i64, b: i64) -> i64 {
    a * b / gcd(a, b)
}

fn convert_fracts(v: Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    let r = v
        .iter()
        .cloned()
        .map(|(i, j)| {
            let x = gcd(i, j);
            return (i / x, j / x);
        })
        .collect::<Vec<_>>();
    let x = r.iter().fold(1, |a, t| lcm(a, t.1));
    r.iter().map(|(i, j)| (i * x / j, x)).collect()
}
