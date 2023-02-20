fn decompose(n: u32) -> (Vec<u32>, u32) {
    let mut v = vec![];
    let mut i: f64 = 2.0;
    let mut t = n as f64;
    while t > 0.0 {
        let x = (t.ln() / i.ln()).floor();
        let p = i.powf(x);
        if x > 1.0 {
            v.push(x as u32);
        } else {
            break;
        }
        t -= p;
        i += 1.0;
    }
    (v, t as u32)
}
