fn f(s: &String) -> u8 {
    [&s[1..3], &s[3..5], &s[5..]]
        .iter()
        .map(|e| u8::from_str_radix(e, 16).ok().unwrap())
        .max()
        .unwrap()
}

fn brightest(c: &[String]) -> String {
    c.iter()
        .rev()
        .max_by(|a, b| f(a).cmp(&f(b)))
        .unwrap()
        .to_string()
}
