use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;

fn simplify(s: &str) -> String {
    let mut d = HashMap::new();
    for e in Regex::new(r"[^a-z]*+[a-z]+").unwrap().find_iter(s) {
        let m = e.as_str();
        let k = Regex::new(r"[^a-z]")
            .unwrap()
            .replace_all(m, "")
            .to_string()
            .chars()
            .sorted()
            .collect::<String>();
        let x = Regex::new(r"[a-z]").unwrap().replace_all(m, "").to_string();
        let v = if x == "-" {
            -1
        } else if x == "" || x == "+" {
            1
        } else {
            x.parse().unwrap()
        };
        d.insert(k.clone(), d.get(&k).unwrap_or(&0) + v);
    }
    d.iter()
        .filter(|t| t.1 != &0)
        .sorted_by_key(|t| (t.0.len(), t.0))
        .map(|t| match t.1 {
            -1 => format!("-{}", t.0),
            1 => t.0.to_string(),
            _ => format!("{}{}", t.1, t.0),
        })
        .join("+")
        .replace("+-", "-")
}
