fn bananas(s: &str) -> Vec<String> {
    let b = ['b', 'a', 'n', 'a', 'n', 'a'];
    let mut r = vec![];
    let mut q = vec![(0, s.to_string(), String::new())];
    while q.len() > 0 {
        let (i, v, t) = q.pop().unwrap();
        if i == 6 {
            r.push(t.to_string() + &"-".repeat(s.len() - t.len()));
            continue;
        }
        if v == "" {
            continue;
        }
        if v.chars().nth(0).unwrap() == b[i] {
            q.push((i + 1, v[1..].to_string(), t.to_string() + &b[i].to_string()));
        }
        q.push((i, v[1..].to_string(), t.to_string() + "-"));
    }
    r
}
