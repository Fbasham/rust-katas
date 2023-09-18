fn reversed_number(mut n: u32, b: u32) -> u64 {
    if b == 1 {
        return n as u64;
    }
    let mut r = 0;
    while n > 0 && b > 1 {
        r = r * b + n % b;
        n /= b;
    }
    r as u64
}
