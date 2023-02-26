fn race(v1: i32, v2: i32, g: i32) -> Option<Vec<i32>> {
    if v1 >= v2 {
        return None;
    }
    let t = (g as f64) / ((v2 - v1) as f64);
    Some(vec![
        t.floor() as i32,
        (t * 60.0 % 60.0 + 1e-6) as i32,
        ((t * 60.0 % 1.0 * 60.0 + 1e-6) % 60.0).floor() as i32,
    ])
}
