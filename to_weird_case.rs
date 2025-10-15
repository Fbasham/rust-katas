fn to_weird_case(s: &str) -> String {
    let mut k = 0;
    let mut t = String::new();
    for e in s.chars() {
        if e.is_alphabetic() {
            t.push(if k % 2 == 0 {
                e.to_ascii_uppercase()
            } else {
                e.to_ascii_lowercase()
            });
            k ^= 1;
        } else {
            t.push(e);
            k = 0;
        }
    }
    t
}
