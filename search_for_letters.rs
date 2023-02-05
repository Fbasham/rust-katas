fn change(s: &str) -> String {
    (0..26)
        .map(|i| {
            if s.to_lowercase().contains(char::from(i + 97)) {
                '1'
            } else {
                '0'
            }
        })
        .collect()
}
