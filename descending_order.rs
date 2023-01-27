fn descending_order(x: u64) -> u64 {
    let mut v = x
        .to_string()
        .chars()
        .map(|e| (e as u32) - 48)
        .collect::<Vec<u32>>();
    v.sort_by(|a, b| b.cmp(a));
    v.iter()
        .map(|e| e.to_string())
        .collect::<Vec<String>>()
        .join("")
        .parse()
        .unwrap()
}
