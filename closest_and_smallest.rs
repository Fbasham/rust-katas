fn closest(s: &str) -> String {
    let a = s
        .trim()
        .split(" ")
        .map(|e| (e, e.chars().map(|x| ((x as u8 - 48) as i32)).sum::<i32>()))
        .collect::<Vec<_>>();
    let mut m = 100000;
    let mut v = vec![];
    for i in 0..a.len() {
        for j in i + 1..a.len() {
            let d = (a[j].1 - a[i].1).abs();
            m = m.min(d);
            let mut t = vec![(d, a[i].1, i, a[i].0), (d, a[j].1, j, a[j].0)];
            t.sort_by_key(|e| e.1);
            v.push(t);
        }
    }
    v = v.into_iter().filter(|u| u[0].0 == m).collect();
    v.sort_by(|a, b| {
        (a[0].1 + a[1].1)
            .cmp(&(b[0].1 + b[1].1))
            .then_with(|| (a[0].2 + a[1].2).cmp(&(b[0].2 + b[1].2)))
    });
    let x = v[0][0];
    let y = v[0][1];
    format!("[({},{},{})({},{},{})]", x.1, x.2, x.3, y.1, y.2, y.3)
}
