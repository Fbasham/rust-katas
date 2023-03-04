fn f(s: String) -> String {
    let mut t = String::new();
    for c in s.chars() {
        if t.is_empty() {
            t = t.to_owned() + &c.to_string();
        } else if c.to_string() != t[t.len() - 1..t.len()] {
            t = t.to_owned() + &c.to_string();
        }
    }
    t
}

fn dup(v: Vec<String>) -> Vec<String> {
    v.iter().map(|s| f(s.to_string())).collect()
}
