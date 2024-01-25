fn power_mod(mut x: u64, mut y: u64, n: u64) -> u64 {
    let mut r = 1;
    x = x % n;
    if x == 0 {
        return 0;
    }
    while y > 0 {
        if y & 1 == 1 {
            r = r * x % n
        }
        y = y >> 1;
        x = x * x % n;
    }
    r % n
}
