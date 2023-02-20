use std::collections::*;

fn delete_nth(a: &[u8], n: usize) -> Vec<u8> {
    let mut d = HashMap::new();
    let mut v = vec![];
    for k in a.iter() {
        if d.get(&k).unwrap_or(&0) < &n {
            v.push(*k);
        }
        d.entry(k).and_modify(|e| *e += 1).or_insert(1);
    }
    v
}
