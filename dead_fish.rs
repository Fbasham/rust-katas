fn parse(s: &str) -> Vec<i32> {
    let mut v = vec![];
    let mut n = 0 as i32;
    for e in s.chars() {
        match e {
            'i' => n += 1,
            'd' => n -= 1,
            's' => n = n.pow(2),
            'o' => v.push(n),
            _ => n += 0,
        }
    }
    v
}
