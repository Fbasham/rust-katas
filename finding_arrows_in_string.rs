use regex::Regex;

pub fn arrow_search(s: &str) -> i64 {
    Regex::new(r"<?=+>?|<?-+>?|<?=+>?|<?-+>?|<|>")
        .unwrap()
        .find_iter(&s)
        .map(|e| {
            let t = e.as_str();
            if (!t.contains(">") && !t.contains("<")) || (t.contains(">") && t.contains("<")) {
                return 0;
            }
            (t.len() as i64)
                * (if t.contains("=") { 2 } else { 1 })
                * (if t.contains(">") { 1 } else { -1 })
        })
        .sum()
}
