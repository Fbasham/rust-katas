fn interval_insert(a: &[(i32, i32)], p: (i32, i32)) -> Vec<(i32, i32)> {
    let mut v = vec![p];
    for t in a {
        let mut f = false;
        for i in 0..v.len() {
            if (t.0 <= v[i].0 && t.1 >= v[i].1)
                || (t.0 >= v[i].0 && t.1 <= v[i].1)
                || (t.0 <= v[i].0 && t.1 >= v[i].0)
                || (t.0 <= v[i].1 && t.1 >= v[i].1)
            {
                v[i] = (v[i].0.min(t.0), v[i].1.max(t.1));
                f = true;
            }
        }
        if f == false {
            v.push((t.0, t.1))
        }
    }
    v.sort();
    v
}
