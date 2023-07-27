fn tops(s: &str) -> String {
    let mut r = String::new();
    let mut x = 2;
    let mut y = 2;
    let mut i = 0;
    while i < s.len() {
        i += x;
        if i > s.len() {
            break;
        }
        r = format!("{}{}", &s[i..(i + y).min(s.len())], r);
        i += y;
        x += 3;
        y += 1;
    }
    r
}
