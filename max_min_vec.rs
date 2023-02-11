fn solve(a: &[i32]) -> Vec<i32> {
    let mut v = a.to_vec();
    v.sort();
    let mut r = vec![];
    while v.len() > 0 {
        r.push(v.pop().unwrap());
        if v.len() > 0 {
            r.push(v.remove(0))
        };
    }
    r
}
