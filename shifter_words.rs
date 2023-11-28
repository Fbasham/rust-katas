use itertools::Itertools;

fn shifter(s: &str) -> usize {
    s.split(" ")
        .filter(|&e| e.len() > 0 && e.chars().all(|x| "HNOISXZWM".contains(x)))
        .unique()
        .count()
}
