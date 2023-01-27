fn capitalize(s: &str) -> Vec<String> {
    fn f(s: String, x: usize) -> String {
        s.chars()
            .enumerate()
            .map(|(i, e)| {
                if i % 2 == x {
                    e.to_ascii_uppercase()
                } else {
                    e.to_ascii_lowercase()
                }
            })
            .collect()
    }
    [f(s.to_string(), 0), f(s.to_string().clone(), 1)].to_vec()
}
