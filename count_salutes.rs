fn count_salutes(s: &str) -> usize {
    let mut t = 0;
    let mut c = 0;
    for e in s.chars() {
        match e {
            '>' => t += 1,
            '<' => c += 2*t,
            _ => ()
        }
    }
    c
}