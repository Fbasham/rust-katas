fn unlock(s: &str) -> String {
    let t = "22233344455566677778889999".chars().collect::<Vec<_>>();
    s.to_lowercase()
        .chars()
        .map(|e| t[e as usize - 97])
        .collect()
}
