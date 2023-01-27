use itertools::Itertools;

fn reverse_words(s: &str) -> String {
    return s.split_whitespace().rev().join(" ");
}
