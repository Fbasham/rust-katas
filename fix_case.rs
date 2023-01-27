fn solve(s: &str) -> String {
    let l = s.chars().filter(|&e| e >= 'a' && e <= 'z').count();
    let u = s.chars().filter(|&e| e >= 'A' && e <= 'Z').count();
    if l >= u {
        s.to_lowercase()
    } else {
        s.to_uppercase()
    }
}
