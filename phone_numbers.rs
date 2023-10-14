use std::collections::HashSet;

fn phone_number(a: &[&str]) -> u32 {
    let mut s = HashSet::new();
    for e in a {
        for i in 1..=e.len() {
            s.insert(&e[..i]);
        }
    }
    s.len() as u32
}
