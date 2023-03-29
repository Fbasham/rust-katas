use itertools::Itertools;
use std::collections::HashMap;

fn ranks(a: &[i32]) -> Vec<usize> {
    let t = a
        .iter()
        .cloned()
        .unique()
        .sorted()
        .rev()
        .collect::<Vec<_>>();
    let d = a.iter().cloned().counts();
    let mut c = HashMap::new();
    let mut p = 1;
    for k in t {
        c.insert(k, p);
        p += d.get(&k).unwrap();
    }
    a.iter()
        .map(|e| c.get(&e).unwrap())
        .map(|e| *e)
        .collect::<Vec<_>>()
}
