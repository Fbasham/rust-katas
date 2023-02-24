fn sq_in_rect(a: i32, b: i32) -> Option<Vec<i32>> {
    if a == b {
        return None;
    }
    let mut v = vec![];
    let mut l = a;
    let mut w = b;
    while l != w {
        let m = l.min(w);
        v.push(m);
        if l != m {
            l -= m;
        }
        if w != m {
            w -= m;
        }
    }
    v.push(l);
    Some(v)
}
