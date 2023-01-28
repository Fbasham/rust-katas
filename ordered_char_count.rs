use itertools::Itertools;

fn ordered_count(s: &str) -> Vec<(char, i32)> {
    s.chars()
        .unique()
        .map(|e| (e, s.matches(e).count() as i32))
        .collect()
}
