fn f(n: u32) -> bool {
    if n % 2 == 0 {
        return n == 2;
    }
    for i in (3..=((n as f64).sqrt() as u32)).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn minimum_number(a: &[u32]) -> u32 {
    let s: u32 = a.iter().sum();
    for i in 0..1000 {
        if f(s + i) {
            return i;
        }
    }
    0
}
