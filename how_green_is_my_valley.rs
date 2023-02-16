fn make_valley(a: Vec<i32>) -> Vec<i32> {
    let mut v = a.clone();
    v.sort();
    let mut l = vec![];
    let mut r = vec![];
    while v.len() > 0 {
        l.push(v.pop().unwrap());
        if v.len() > 0 {
            r.insert(0, v.pop().unwrap())
        }
    }
    l.append(&mut r);
    l
}
