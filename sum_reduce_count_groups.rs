fn sum_groups(a: &[u32]) -> usize {
    let mut v = a.to_vec();
    let mut p = vec![];
    while p != v {
        p = v.clone();
        let mut t = 1001;
        let mut n = 0;
        let mut u = vec![];
        for e in v {
            if t == 1001 || t % 2 == e % 2 {
                n += e;
                t = e;
            } else {
                u.push(n);
                t = e;
                n = e;
            }
        }
        u.push(n);
        v = u.clone();
    }
    v.len()
}
