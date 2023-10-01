use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn performant_smallest(a: &[u32], n: usize) -> Vec<u32> {
    let mut q = BinaryHeap::new();
    for (i, e) in a.iter().enumerate() {
        q.push(Reverse((e, i)));
    }
    let mut t = vec![];
    for _ in 0..n {
        t.push(q.pop().unwrap().0)
    }
    t.sort_by_key(|t| t.1);
    t.iter().map(|t| *t.0).collect()
}
