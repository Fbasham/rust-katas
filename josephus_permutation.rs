fn josephus<T: Clone + Copy>(xs: Vec<T>, k: usize) -> Vec<T> {
    let mut v = xs.clone();
    let mut r = vec![];
    let mut i = (k - 1) % v.len();
    while v.len() > 0 {
        r.push(v.remove(i));
        i = ((i + k - 1) % v.len().max(1)).max(0);
    }
    r
}
