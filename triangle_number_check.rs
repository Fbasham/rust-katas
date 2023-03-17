fn is_triangle_number(n: u64) -> bool {
    let x = ((-1.0 + ((1 + 8 * n) as f64).sqrt()) / 2.0) as u64;
    x * (x + 1) / 2 == n
}
