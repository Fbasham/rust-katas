#[allow(unused)]
use crate::preloaded::{build_tree, visualize_tree};
use std::collections::HashSet;

fn count_skills(a: &[usize], s: &HashSet<usize>) -> usize {
    if s.is_empty() {
        return 0;
    }
    let mut r = HashSet::new();
    for mut e in s {
        while e != &0 {
            r.insert(e);
            e = &a[*e];
            if s.contains(e) {
                break;
            }
        }
    }
    r.len() + 1
}
