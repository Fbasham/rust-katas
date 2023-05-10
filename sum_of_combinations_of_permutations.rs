use itertools::Itertools;
use std::collections::HashSet;

fn sc_perm_comb(n: u32) -> u64 {
    let s = n.to_string();
    let mut d = HashSet::new();
    for i in 1..=s.len() {
        for p in s.chars().permutations(i) {
            let e = p.iter().collect::<String>().parse::<u64>().unwrap();
            d.insert(e);
        }
    }
    d.iter().sum()
}
