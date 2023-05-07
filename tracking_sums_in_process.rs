use itertools::Itertools;

fn track_sum(a: &[i32]) -> (Vec<i32>, Vec<i32>) {
    let mut r = vec![a.iter().cloned().sum()];
    let mut v = a.iter().cloned().unique().collect::<Vec<_>>();
    r.push(v.iter().sum());
    v = v.iter().cloned().sorted().rev().collect();
    v = v.iter().zip(&v[1..]).map(|t| t.0 - t.1).collect();
    r.push(v.iter().sum());
    v = v.iter().cloned().unique().collect();
    r.push(v.iter().sum());
    (r, v)
}
