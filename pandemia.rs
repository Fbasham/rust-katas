fn infected(s: &str) -> f64 {
    let p = s.chars().filter(|&e| e == '0' || e == '1').count() as f64;
    let i: f64 = s
        .split('X')
        .map(|e| if e.contains('1') { e.len() } else { 0 } as f64)
        .sum();
    100.0 * i / p.max(1.0)
}
