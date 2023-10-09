fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn reduce_fraction(t: (u32, u32)) -> (u32, u32) {
    let g = gcd(t.0, t.1);
    (t.0 / g, t.1 / g)
}
