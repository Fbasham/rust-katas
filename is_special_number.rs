fn special_number(n: u64) -> String {
    match n.to_string().chars().all(|e| (e as u32) < 54) {
        true => "Special!!".to_string(),
        false => "NOT!!".to_string(),
    }
}
