use itertools::Itertools;
use std::collections::HashSet;

fn f(n: &u32) -> i32 {
    let mut d = HashSet::new();
    let s = n.to_string();
    for p in s.chars().permutations(s.len()) {
        let t = p.iter().join("").parse::<f64>().unwrap();
        if t.sqrt()%1.0==0.0 {
            d.insert(t as i32);
        }
    }
    -(d.len() as i32)
}

fn sort_by_perfsq(a: &[u32]) -> Vec<u32> {
    a.iter().cloned().sorted_by_key(|e| (f(e),e.clone())).collect::<Vec<_>>()
}