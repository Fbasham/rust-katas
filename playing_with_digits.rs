fn dig_pow(n: i64, p: i32) -> i64 {
    let m = n
        .to_string()
        .chars()
        .enumerate()
        .map(|(i, e)| (e as i64 - 48).pow(i as u32 + p as u32))
        .sum::<i64>();
    let d = m / n;
    if d * n == m {
        d
    } else {
        -1
    }
}
