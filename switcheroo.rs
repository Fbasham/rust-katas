fn switcheroo(s: &str) -> String {
    s.chars()
        .map(|e| {
            if e == 'a' {
                'b'
            } else if e == 'b' {
                'a'
            } else {
                e
            }
        })
        .collect()
}
