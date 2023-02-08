fn gcd(a: u8, b: u8) -> u8 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn are_coprime(n: u8, m: u8) -> bool {
    gcd(n, m) == 1
}
