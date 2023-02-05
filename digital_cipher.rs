fn encode(s: String, n: i32) -> Vec<i32> {
    let k = n
        .to_string()
        .chars()
        .map(|e| e as i32 - 48)
        .collect::<Vec<i32>>();
    s.chars()
        .enumerate()
        .map(|(i, e)| (e as i32 - 96) + k[i % k.len()])
        .collect()
}
