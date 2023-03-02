fn run_length_encoding(s: &str) -> Vec<(usize, char)> {
    let mut v = vec![];
    let mut p = ' ';
    let mut c = 0;
    for e in (s.to_owned() + " ").chars() {
        if p != e {
            if c > 0 {
                v.push((c, p));
            }
            p = e;
            c = 1;
        } else {
            c += 1;
        }
    }
    v
}
