use std::collections::HashMap;

fn count(s: &str) -> HashMap<char,i32> {
    let mut d = HashMap::new();
    for k in s.chars() {
        d.insert(k,d.get(&k).unwrap_or(&0)+1);
    }
    d
}