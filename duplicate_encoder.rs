use itertools::Itertools;

fn duplicate_encode(s: &str) -> String {
    let d = s.to_lowercase().chars().counts();
    s.to_lowercase()
        .chars()
        .map(|k| if d[&k] == 1 { '(' } else { ')' })
        .collect()
}
