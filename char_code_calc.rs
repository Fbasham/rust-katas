fn calc(s: &str) -> u32 {
    let t = s.chars().map(|e| (e as u8).to_string()).collect::<String>();
    let f = |s: String| s.chars().map(|e| (e as u32) - 48).sum::<u32>();
    f(t.clone()) - f(t.clone().replace("7", "1"))
}
