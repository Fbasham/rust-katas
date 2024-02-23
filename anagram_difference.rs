use itertools::Itertools;

fn anagram_difference(s1: &str, s2: &str) -> u32 {
    let d1 = s1.chars().counts();
    let d2 = s2.chars().counts();
    (s1.to_owned()+s2).chars().unique().map(|e| {
        let x = *d1.get(&e).unwrap_or(&0) as u32;
        let y = *d2.get(&e).unwrap_or(&0) as u32;
        let m = x.min(y);
        x.saturating_sub(m)+y.saturating_sub(m)
    }).sum()
}