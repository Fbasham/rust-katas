use lazy_static::lazy_static;
use std::collections::HashSet;
use itertools::Itertools;

lazy_static! {
    static ref A: Vec<u64> = {
        let mut v = HashSet::new();
        for i in 0..300 {
            for j in 0..100 {
                for k in 0..30 {
                    v.insert(2_u64.saturating_pow(i).saturating_mul(3_u64.saturating_pow(j)).saturating_mul(5_u64.saturating_pow(k)));
                }
            }
        }
        v.iter().sorted().cloned().collect()
    };
}

fn hamming(n: usize) -> u64 {
    A[n-1]
}