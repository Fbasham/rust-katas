fn tower_builder(n: usize, b: (usize, usize)) -> Vec<String> {
    let mut v = vec![];
    let k = (2 * n - 1) * b.0;
    for i in 0..n {
        for _ in 0..b.1 {
            let t = "*".repeat((2 * i + 1) * b.0);
            let x = " ".repeat((k - t.len()) / 2);
            v.push(format!("{}{}{}", x, t, x));
        }
    }
    v
}
