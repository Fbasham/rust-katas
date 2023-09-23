use itertools::Itertools;

fn is_pangram(s: &str) -> bool {
    let t = "abcdefghijklmnopqrstuvwxyz";
    s.to_lowercase()
        .chars()
        .filter(|e| e.is_alphabetic())
        .unique()
        .counts()
        == t.chars().counts()
}
