fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn coprimes(n: u32) -> Vec<u32> {
    (1..n).filter(|&i| gcd(i, n) == 1).collect()
}
