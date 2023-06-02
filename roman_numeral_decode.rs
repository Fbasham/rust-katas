fn roman_as_num(t: &str) -> u64 {
    let d = vec![
        ("M", 1000),
        ("CM", 900),
        ("D", 500),
        ("CD", 400),
        ("C", 100),
        ("XC", 90),
        ("L", 50),
        ("XL", 40),
        ("X", 10),
        ("IX", 9),
        ("V", 5),
        ("IV", 4),
        ("I", 1),
    ];
    let mut r = 0;
    let mut s = t.to_string();
    for (k, v) in d {
        while s.starts_with(k) {
            r += v;
            s = s[k.len()..].to_string();
        }
    }
    r
}
