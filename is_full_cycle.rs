use std::collections::HashSet;

fn full_cycle(a: &[usize]) -> bool {
    let mut s = HashSet::new();
    let mut i = 0;
    while !s.contains(&i) {
        s.insert(i);
        i = a[i];
        if s.contains(&i) {
            break;
        }
    }
    s.len() == a.len()
}
