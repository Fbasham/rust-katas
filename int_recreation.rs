use itertools::Itertools;

fn prod2sum(a: i64, b: i64, c: i64, d: i64) -> Vec<(i64, i64)> {
    let k = (a * a + b * b) * (c * c + d * d);
    let mut v = vec![];
    let t = vec![
        (a * b + c * d).abs(),
        (a * b - c * d).abs(),
        (a * c + b * d).abs(),
        (a * c - b * d).abs(),
        (a * d + b * c).abs(),
        (a * d - b * c).abs(),
        (b * d + a * c).abs(),
        (b * d - a * c).abs(),
    ]
    .iter()
    .cloned()
    .unique()
    .sorted()
    .collect::<Vec<_>>();
    for i in 0..t.len() {
        for j in i..t.len() {
            let x = t[i];
            let y = t[j];
            if x * x + y * y == k {
                v.push((x, y));
            }
        }
    }
    v
}
