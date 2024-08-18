pub fn max_depth(s: String) -> u8 {
    let mut c = 0;
    let mut m = 0;
    for e in s.chars() {
        if e == '(' {
            c += 1
        }
        if e == ')' {
            c -= 1
        }
        m = m.max(c);
    }
    m
}
