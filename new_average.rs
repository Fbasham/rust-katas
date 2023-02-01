fn new_avg(a: &[f64], n: f64) -> Option<i32> {
    let r = (n * ((a.len() as f64) + 1.0) - a.iter().sum::<f64>()).round() as i32;
    match r < 0 {
        true => None,
        false => Some(r),
    }
}
