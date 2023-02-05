fn max_gap(a: &[i32]) -> i32 {
    let mut t = a.clone().to_owned();
    t.sort();
    t.windows(2).map(|e| (e[0] - e[1]).abs()).max().unwrap()
}
