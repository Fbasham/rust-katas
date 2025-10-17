fn to_roman(mut n: u32) -> String {
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
    let mut r = String::new();
    for (k, v) in d {
        r += &k.repeat((n / v) as usize);
        n %= v;
    }
    r
}

fn from_roman(mut s: &str) -> u32 {
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
    while s.len() > 0 {
        let (k, v) = d.iter().find(|(k, _)| s.starts_with(k)).unwrap();
        r += v;
        s = &s[k.len()..];
    }
    r
}
