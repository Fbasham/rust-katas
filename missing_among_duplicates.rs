use std::collections::HashSet;
use itertools::Itertools;

fn find_dups_miss(a: &[u32]) -> (u32, Vec<u32>) {
    let mut d = HashSet::new();
    let mut v = HashSet::new();
    let mut m: f64 = 1e9;
    let mut n: f64 = 0.0;
    for e in a {
        if d.contains(e) {
            v.insert(e);
        }
        d.insert(e);
        m = m.min(*e as f64);
        n = n.max(*e as f64);
    }
    let s = d.into_iter().map(|e| *e as f64).sum::<f64>();
    let x = v.into_iter().cloned().sorted().collect::<Vec<_>>();
    (((n-m+1.0)/2.0*(m+n)-s) as u32,x)
}