fn find(s: &str) -> u32 {
    for i in 1..s.len() + 1 {
        let mut t = s[..i].parse::<u32>().unwrap();
        let v = t.clone();
        let mut r = String::new();
        while r.len() < s.len() {
            r += &t.to_string();
            t += 1;
        }
        if r == s {
            return v;
        }
    }
    0
}
