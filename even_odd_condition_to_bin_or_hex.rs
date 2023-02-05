fn evens_and_odds(n: u64) -> String {
    match n % 2 == 0 {
        true => format!("{:b}", n).to_string(),
        false => format!("{:x}", n).to_string(),
    }
}
