fn rev_sub(a: &[i32]) -> Vec<i32> {
    let mut t = vec![];
    let mut r = vec![];
    for e in a {
        if e % 2 == 0 {
            t.insert(0, *e);
        } else {
            r.extend(t);
            r.push(*e);
            t = vec![];
        }
    }
    r.extend(t);
    r
}
