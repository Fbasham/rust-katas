use itertools::Itertools;
use lazy_static::lazy_static;
use std::cmp::Reverse;
use std::collections::*;

lazy_static! {
    static ref A: Vec<i64> = {
        let mut v = HashSet::new();
        let mut q = BinaryHeap::from([Reverse(1)]);
        while v.len() < 100000 {
            let e = q.pop().unwrap().0;
            if !v.contains(&e) {
                v.insert(e);
                let x = 2 * e + 1;
                let y = 3 * e + 1;
                if !v.contains(&x) {
                    q.push(Reverse(x));
                }
                if !v.contains(&y) {
                    q.push(Reverse(y));
                }
            }
        }
        v.iter().cloned().sorted().collect()
    };
}

fn dbl_linear(n: u32) -> u32 {
    A[n as usize] as u32
}
