fn number_joy(n: u32) -> bool {
    let s: u32 = n
        .to_string()
        .chars()
        .map(|e| e.to_string().parse::<u32>().unwrap())
        .sum();
    s * (s
        .clone()
        .to_string()
        .chars()
        .rev()
        .collect::<String>()
        .parse::<u32>()
        .unwrap())
        == n
}
