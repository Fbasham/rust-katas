use itertools::Itertools;
use std::collections::HashMap;

pub fn find_uniq<'a>(a: &'a [&str]) -> &'a str {
    let mut d = HashMap::new();
    for s in a {
        let k = s
            .replace(" ", "")
            .to_lowercase()
            .chars()
            .unique()
            .sorted()
            .join("");
        d.entry(k).or_insert(vec![]).push(s);
    }
    d.values().find(|t| t.len() == 1).unwrap()[0]
}
