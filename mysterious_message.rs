use itertools::Itertools;
use regex::Regex;

pub fn decipher_message(code: &str) -> String {
    let mut r = vec![];
    for e in Regex::new(r"[^A-Z\-]")
        .unwrap()
        .replace_all(&code.to_uppercase(), "")
        .split("-")
    {
        let mut w = e.chars().unique().collect::<Vec<char>>();
        w.sort_by(|a, b| {
            e.chars()
                .filter(|&x| &x == b)
                .count()
                .cmp(&e.chars().filter(|&x| &x == a).count())
        });
        r.push(w.iter().join(""));
    }
    r.iter().filter(|&e| e != "").join(" ")
}
