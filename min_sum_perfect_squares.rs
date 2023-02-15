fn sum_of_squares(n: u64) -> u64 {
    let mut m = 100000;
    for x in (1..=(n as f64).sqrt() as u64).rev() {
        let mut t = n;
        let mut c = 0;
        let mut i = x;
        while t > 0 {
            let k = t / (i * i);
            t = t % (i * i);
            c += k;
            i = (t as f64).sqrt() as u64;
        }
        m = m.min(c);
    }
    m
}
