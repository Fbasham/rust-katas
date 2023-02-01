fn contain_all_rots(s: &str, v: Vec<&str>) -> bool {
    for i in 0..s.len() {
        let t = format!("{}{}", &s[i..], &s[..i]);
        if !v.contains(&t.as_str()) {
            return false;
        }
    }
    true
}
