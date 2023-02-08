use itertools::Itertools;

fn solve(s: &str) -> bool {
    let t = s.chars().sorted().collect::<String>();
    "abcdefghijklmnopqrstuvwxyz".contains(&t)
}
