fn movie(c: i32, t: i32, p: f64) -> i32 {
    let mut n = c as f64;
    for i in 1.. {
        n += (t as f64) * p.powf(i as f64);
        if (t as f64) * (i as f64) > n.ceil() {
            return i;
        }
    }
    0
}
