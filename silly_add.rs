fn add(n1: u32, n2: u32) -> u64 {
    let mut a = n1;
    let mut b = n2;
    let mut s = String::new();
    while a > 0 || b > 0 {
        s = format!("{}{}", a % 10 + b % 10, s);
        a /= 10;
        b /= 10;
    }
    s.parse().unwrap_or(0)
}
