use itertools::Itertools;

fn number_of_pairs(a: &[&str]) -> u32 {
    a.iter().counts().values().map(|e| (e / 2) as u32).sum()
}
