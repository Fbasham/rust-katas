pub fn ideal_trader(a: &[f64]) -> f64 {
    let mut p = 1.0;
    for i in 1..a.len() {
        if a[i] > a[i - 1] {
            p *= a[i] / a[i - 1];
        }
    }
    return p;
}
