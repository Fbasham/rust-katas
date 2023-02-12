fn find_next_power(n: u64, p: u32) -> u64 {
    let mut v = (n as f64).powf(1.0 / (p as f64));
    if (v - v.ceil()).abs() < 1e-6 {
        v += 1.0
    };
    (v.ceil() as u64).pow(p)
}
