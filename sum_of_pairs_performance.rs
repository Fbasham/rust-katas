use std::collections::HashSet;

fn sum_pairs(a: &[i8], n: i8) -> Option<(i8, i8)> {
    let mut s = HashSet::new();
    for e in a {
        let k = n - e;
        if s.contains(&k) {
            return Some((k, *e));
        }
        s.insert(e);
    }
    None
}
