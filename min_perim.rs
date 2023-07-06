fn minimum_perimeter(a: u64) -> u64 {
    let n = (a as f64).sqrt().ceil() as u64;
    for i in (1..=n).rev() {
        let j = a/i;
        if i*j == a {
            return 2*i+2*j;
        }
    }
    0
}