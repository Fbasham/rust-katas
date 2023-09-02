use itertools::Itertools;

fn gangs(a: &[u32], k: u32) -> u32 {
    (1..=k)
        .map(|i| {
            a.iter()
                .filter(|&e| i % e == 0)
                .map(|e| e.to_string())
                .join(" ")
        })
        .unique()
        .count() as u32
}
