fn num_as_roman(n: i32) -> String {
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
    let mut t = n;
    for (k, v) in d {
        if t >= v {
            let x = t / v;
            r += &k.repeat(x as usize);
            t -= v * x;
        }
    }
    r
}
