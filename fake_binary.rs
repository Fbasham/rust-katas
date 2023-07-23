fn fake_bin(s: &str) -> String {
    s.chars()
        .map(|e| match e < '5' {
            true => '0',
            _ => '1',
        })
        .collect()
}
