fn remove_nb(n: i32) -> Vec<(i32, i32)> {
    let s = (n as f64) * (n as f64 + 1.0) / 2.0;
    let mut i = s.sqrt().floor();
    let mut v = vec![];
    while i > 0.0 {
        let j = ((s - i) / (i + 1.0)).floor();
        if j > 0.0 && j < n as f64 && (i * j) == s - i - j {
            v.push((i as i32, j as i32));
            v.push((j as i32, i as i32));
        }
        i -= 1.0;
    }
    v.sort_by_key(|t| t.0);
    v
}
