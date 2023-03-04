use itertools::Itertools;

fn find_children(s: &str) -> String {
    s.chars()
        .sorted_by(|a, b| {
            a.to_ascii_lowercase()
                .cmp(&b.to_ascii_lowercase())
                .then_with(|| a.cmp(&b))
        })
        .collect()
}
