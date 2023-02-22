fn revrot(s: &str, c: usize) -> String {
    let mut r = String::new();
    if c == 0 || c > s.len() {
        return r;
    }
    for i in (0..s.len()).step_by(c) {
        if i + c > s.len() {
            break;
        }
        let t = &s[i..i + c].to_string();
        let n = t.chars().map(|e| (e as i32).pow(3)).sum::<i32>();
        r = format!(
            "{}{}",
            r,
            match n & 1 == 0 {
                true => t.chars().rev().collect::<String>(),
                false => format!("{}{}", &t[1..], &t[..1]),
            }
        )
    }
    r
}
