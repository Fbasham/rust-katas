fn count_photos(s: &str) -> usize {
    let mut c = 0;
    let mut r = 0;
    let mut t = 0;
    for e in s.chars() {
        match e {
            '<' => t += c,
            '>' => r += 1,
            _ => {
                c += 1;
                t += r
            }
        }
    }
    t
}
