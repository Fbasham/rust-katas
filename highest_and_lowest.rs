fn high_and_low(s: &str) -> String {
    let a = s
        .split_whitespace()
        .map(|e| e.parse::<i32>().unwrap())
        .min()
        .unwrap()
        .to_string();
    let b = s
        .split_whitespace()
        .map(|e| e.parse::<i32>().unwrap())
        .max()
        .unwrap()
        .to_string();
    b + " " + &a
}
