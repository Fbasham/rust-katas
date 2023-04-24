use regex::Regex;

fn is_valid_message(s: &str) -> bool {
    if !s.is_empty() && !Regex::new(r"^((\d+)([a-zA-Z]+))+$").unwrap().is_match(s) {
        return false;
    }
    Regex::new(r"(\d+)([a-zA-Z]+)")
        .unwrap()
        .captures_iter(s)
        .all(|e| {
            e.get(2).unwrap().as_str().len() == e.get(1).unwrap().as_str().parse::<usize>().unwrap()
        })
}
