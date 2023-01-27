fn square_digits(n: u64) -> u64 {
    n.to_string()
        .chars()
        .map(|e| e.to_digit(10).unwrap().pow(2).to_string())
        .collect::<Vec<String>>()
        .join("")
        .parse()
        .unwrap()
}
