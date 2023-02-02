fn tidy_number(n: u64) -> bool {
    let s = n
        .to_string()
        .chars()
        .map(|e| e as u32)
        .collect::<Vec<u32>>();
    s.iter().zip(s.iter().skip(1)).all(|(i, j)| j >= i)
}
