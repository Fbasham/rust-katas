fn previous_power_of_2(n: i32) -> i32 {
    if n == 1 {
        return 0;
    }
    let mut e = (n.abs() as f64).log2().ceil() as u32;
    if n > 0 {
        e -= 1
    }
    if n < 0 && 2_i64.pow(e) == (n as i64).abs() {
        e += 1
    }
    ((if n < 0 { -1 } else { 1 }) * 2_i64.pow(e)) as i32
}
