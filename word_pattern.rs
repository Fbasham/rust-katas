use std::collections::*;

fn word_pattern(s: &str) -> String {
    let mut d = HashMap::new();
    let mut r = vec![];
    let mut v = 0;
    for k in s.to_lowercase().chars() {
        if !d.contains_key(&k) {
            d.insert(k,v.to_string());
            v += 1;
        }
        r.push(d.get(&k).unwrap().clone());
    }
    r.join(".")
}