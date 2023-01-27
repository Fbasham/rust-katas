use itertools::Itertools;

fn remove_duplicate_words(s: &str) -> String {
    s.split_whitespace().unique().join(" ")
}
