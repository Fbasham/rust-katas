fn max_ball(v0: i32) -> i32 {
    let v = (v0 as f64) / 3.6;
    let mut t = 0.0;
    let mut h = 0.0;
    for i in 0..1000 {
        let x = v * (i as f64) / 10.0 - 9.81 / 2.0 * (i as f64) * (i as f64) / 100.0;
        if x > h {
            h = x;
            t = i as f64;
        }
    }
    t as i32
}
