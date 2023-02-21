fn longest_repetition(s: &str) -> Option<(char, usize)> {
    if s.is_empty() {
        return None;
    }
    let mut c = ' ';
    let mut p = ' ';
    let mut m = 0;
    let mut t = 0;
    for e in (s.to_string() + " ").chars() {
        if e == p {
            t += 1;
        } else {
            if t > m {
                m = t;
                c = p;
            }
            p = e;
            t = 1;
        }
    }
    Some((c, m))
}
