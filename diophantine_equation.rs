fn solequa(n: u64) -> Vec<(u64, u64)> {
    let mut v = vec![];
    for i in 1..=((n as f64).sqrt() as u64 + 1) {
        let j = n as f64 / i as f64;
        if j % 1.0 == 0.0 {
            let x = (i as f64 + j) / 2.0;
            let y = (j - i as f64) / 4.0;
            if x % 1.0 == 0.0 && y % 1.0 == 0.0 {
                v.push((x as u64, y as u64))
            }
        }
    }
    v
}
