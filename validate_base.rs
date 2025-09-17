fn validate_base(s: &str, b: u32) -> bool {
    let t = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    s.chars().all(|e| t[..b as usize].contains(e))
}