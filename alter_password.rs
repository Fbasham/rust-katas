fn play_pass(s: &str, n: u32) -> String {
    let t = s
        .chars()
        .enumerate()
        .map(|(i, c)| match c {
            c if c.is_alphabetic() && i % 2 == 1 => {
                (((c.to_lowercase().next().unwrap() as u8 - 97 + n as u8) % 26) + 97) as char
            }
            c if c.is_alphabetic() && i % 2 == 0 => {
                (((c.to_uppercase().next().unwrap() as u8 - 65 + n as u8) % 26) + 65) as char
            }
            c if c.is_numeric() => ((9 - (c as u8 - 48)) + 48) as char,
            _ => c,
        })
        .collect::<Vec<_>>();
    t.iter().rev().collect::<String>()
}
