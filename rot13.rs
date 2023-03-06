use std::char;

fn rot13(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            c if c >= 'a' && c <= 'z' => {
                char::from_u32((((c as u32 - 97) + 13) % 26) + 97).unwrap()
            }
            c if c >= 'A' && c <= 'Z' => {
                char::from_u32((((c as u32 - 65) + 13) % 26) + 65).unwrap()
            }
            _ => c,
        })
        .collect()
}
