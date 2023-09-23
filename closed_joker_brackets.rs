fn closed_brackets(s: &str) -> bool {
    let mut x = 0;
    let mut y = 0;
    for e in s.chars() {
        if e == '(' {
            x += 1;
            y += 1;
        }
        if e == ')' {
            x = (x - 1).max(0);
            y -= 1;
            if y < 0 {
                break;
            }
        }
        if e == 'J' {
            x = (x - 1).max(0);
            y += 1;
        }
    }
    x <= 0 && y >= 0
}
