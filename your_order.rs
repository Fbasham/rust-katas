use itertools::Itertools;
use regex::Regex;

fn order(s: &str) -> String {
    let f = |s| {
        Regex::new(r"\D")
            .unwrap()
            .replace_all(s, "")
            .parse::<u8>()
            .unwrap()
    };
    s.split_whitespace()
        .sorted_by(|a, b| f(a).cmp(&f(b)))
        .join(" ")
}
