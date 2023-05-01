fn first_n_smallest(a: &[i32], n: usize) -> Vec<i32> {
    let mut v = a.to_vec();
    v.sort();
    v = v[..n].to_vec();
    let mut r = vec![];
    for e in a {
        if v.contains(e) {
            r.push(*e);
            v.remove(v.iter().position(|x| x == e).unwrap());
        }
    }
    r
}
