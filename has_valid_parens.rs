fn valid_parentheses(s: &str) -> bool {
    let mut c = 0;
    for e in s.chars() {
        if e == '(' {
            c += 1
        }
        if e == ')' {
            c -= 1
        }
        if c < 0 {
            return false;
        }
    }
    c == 0
}
