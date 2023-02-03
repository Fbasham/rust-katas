use std::collections::HashSet;

fn count_duplicates(s: &str) -> u32 {
    let mut u = HashSet::new();
    let mut w = HashSet::new();
    let mut c = 0;
    for e in s.to_lowercase().chars() {
        if u.contains(&e) {
            w.insert(e);
        }
        u.insert(e);
    }
    w.len() as u32
}
