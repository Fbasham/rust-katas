use itertools::Itertools;

fn longest(a1: &str, a2: &str) -> String {
    (a1.to_string() + &a2.to_string())
        .chars()
        .sorted()
        .unique()
        .collect()
}
