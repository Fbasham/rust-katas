fn len_curve(n: i32) -> f64 {
    let h = 1.0 / (n as f64);
    (0..n)
        .map(|i| {
            let x1 = (i as f64) * h;
            let y1 = x1 * x1;
            let x2 = (i as f64 + 1.0) * h;
            let y2 = x2 * x2;
            ((x1 - x2).powf(2.0) + (y1 - y2).powf(2.0)).sqrt()
        })
        .sum()
}
