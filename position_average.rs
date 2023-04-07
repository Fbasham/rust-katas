fn pos_average(s: &str) -> f64 {
    let mut c = 0.0;
    let mut n = 0.0;
    let v = s.split(", ").collect::<Vec<_>>();
    for i in 0..v.len() {
        for j in i + 1..v.len() {
            v[i].chars().zip(v[j].chars()).for_each(|(i, j)| {
                c = if i == j { c + 1.0 } else { c };
                n += 1.0;
            })
        }
    }
    (1000000000000.0 * c / n as f64).round() / 10000000000.0
}
