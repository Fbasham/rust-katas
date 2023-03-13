fn gcd(a: u16, b: u16) -> u16 {
    if b > 0 {
        gcd(b, a % b)
    } else {
        a
    }
}

fn nbr_of_laps(x: u16, y: u16) -> (u16, u16) {
    let g = gcd(x, y);
    (y / g, x / g)
}
