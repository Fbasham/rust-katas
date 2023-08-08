fn min_distance(n: u32) -> u32 {
    let t = (1..=n).filter(|i| n % i == 0);
    let v = t.clone().skip(1);
    t.zip(v).map(|(i, j)| j - i).min().unwrap_or(0)
}
