fn valid_parentheses(s: &str) -> bool {
    let mut c = 0;
    for e in s.chars() {
        match e {
            ')' => c -= 1,
            _ => c += 1,
        }
        if c < 0 {
            return false;
        }
    }
    return c == 0;
}
