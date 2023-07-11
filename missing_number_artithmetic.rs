use itertools::Itertools;

fn find_missing_number(a: &[i32]) -> i32 {
    let mut v = a.iter().cloned().sorted().collect::<Vec<_>>();
    let mut t = vec![];
    let mut d = 0.0;
    for i in 0..v.len() {
        let x = v[i + 1] - v[i];
        if t.contains(&x) {
            d = x as f64;
            break;
        }
        t.push(x);
    }
    let n = (v.len() + 1) as f64;
    (n / 2.0 * (2.0 * v[0] as f64 + (n - 1.0) * d)
        - v.iter().cloned().map(|e| e as f64).sum::<f64>()) as i32
}
