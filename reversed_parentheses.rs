fn solve(s: &str) -> Option<usize> {
    let mut c = 0;
    let mut v = vec![];
    for e in s.chars() {
        if e == '(' {
            v.push(e)
        } else if v.len() > 0 {
            v.pop();
        } else {
            v.push('(');
            c += 1;
        }
    }
    if s.len() % 2 == 1 {
        None
    } else {
        Some(c + v.len() / 2)
    }
}
