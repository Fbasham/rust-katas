use itertools::Itertools;
use regex::Regex;

fn dead_ant_count(ants: &str) -> u32 {
    *Regex::new("[^ant]")
        .unwrap()
        .replace_all(&ants.replace("ant", " "), "")
        .chars()
        .counts()
        .values()
        .max()
        .unwrap_or(&0) as u32
}
