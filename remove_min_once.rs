fn remove_smallest(a: &[u32]) -> Vec<u32> {
    let mut v = vec![];
    if a.len() == 0 {
        return v;
    }
    let m = a.iter().min().unwrap();
    let mut f = false;
    for i in a {
        if i == m && f == false {
            f = true;
            continue;
        }
        v.push(*i);
    }
    v
}
