use regex::Regex;

fn alphanumeric(s: &str) -> bool {
    Regex::new(r"^[a-zA-Z\d]+$").unwrap().is_match(s)
}