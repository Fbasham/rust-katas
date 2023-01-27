use regex::Regex;

fn validate_pin(s: &str) -> bool {
    Regex::new("^\\d{4}$|^\\d{6}$").unwrap().is_match(s)
}
