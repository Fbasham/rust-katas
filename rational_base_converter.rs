fn converter(mut n: f64, d: u8, b: f64) -> String {
    let abc: Vec<char> = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
    let k = ((0..).find(|i| b.powf(*i as f64) > n.abs()).unwrap() - 1).max(0);
    let mut r = (if n < 0.0 { "-" } else { "" }).to_string();
    n = n.abs();
    for i in (0..k + 1).rev() {
        r = format!("{r}{}", abc[(n / b.powf(i as f64)) as usize]);
        n %= b.powf(i as f64);
    }
    r = if d > 0 { r + "." } else { r };
    for i in 1..d + 1 {
        r = format!("{r}{}", abc[(n / b.powf(-1.0 * i as f64)) as usize]);
        n %= b.powf(-1.0 * i as f64);
    }
    r
}
