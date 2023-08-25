fn decompose(n: u32, d: u32) -> String {
    let mut x = n as f64;
    let mut y = d as f64;
    let mut v = vec![];
    if x >= y {
        v.push(format!("{}", (x / y).floor()));
        x -= (x / y).floor() * y;
    }
    for _ in 0.. {
        if x == 0.0 || y.is_infinite() {
            break;
        }
        let t = (y / x).ceil();
        v.push(if t == 1.0 {
            "1".to_string()
        } else {
            format!("1/{}", t)
        });
        (x, y) = ((-y + y * x) % x, y * t);
    }
    v.join(", ")
}
