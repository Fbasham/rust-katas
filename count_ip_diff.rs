fn ips_between(a: &str, b: &str) -> u32 {
    let f = |s: String| {
        s.split(".")
            .map(|e| e.parse::<i64>().unwrap())
            .collect::<Vec<_>>()
    };
    f(b.to_string())
        .iter()
        .zip(f(a.to_string()))
        .enumerate()
        .map(|(x, (i, j))| ((i - j) * 256_i64.pow(3 - x as u32)) as i64)
        .sum::<i64>() as u32
}
