fn letters_to_numbers(s: &str) -> usize {
    s.chars().map(|c| if c.is_ascii_lowercase() {c as usize-96} else if c.is_ascii_uppercase() {2*(c as usize-64)} else if c.is_ascii_digit() {c as usize-48} else {0}).sum()
}