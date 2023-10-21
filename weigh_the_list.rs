fn weigh_the_list(a: Vec<i64>) -> Vec<i64> {
    let mut r = vec![];
    for i in (0..a.len()).step_by(2) {
        r.push(-a[i + 1]);
        r.push(a[i]);
    }
    r
}
