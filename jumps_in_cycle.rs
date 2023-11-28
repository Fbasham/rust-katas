fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn get_jumps(v: Vec<i32>, k: i32) -> i32 {
    v.len() as i32 / gcd(v.len() as i32, k)
}
