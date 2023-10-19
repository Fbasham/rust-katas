fn counting_valleys(s: &str) -> u32 {
    let mut c = 0;
    let mut d = 0;
    for e in s.chars() {
        match e {
            'U' => {
                d += 1;
                if d == 0 {
                    c += 1;
                }
            }
            'D' => {
                d -= 1;
            }
            _ => (),
        }
    }
    c
}
