fn array_diff<T: PartialEq>(a: Vec<T>, b: Vec<T>) -> Vec<T> {
    let mut v = vec![];
    for e in a {
        if !b.contains(&e) {
            v.push(e);
        }
    }
    v
}
