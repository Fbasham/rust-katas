use regex::Regex;

fn valid_braces(s: &str) -> bool {
    let re = Regex::new(r"\[\]|\{\}|\(\)").unwrap();
    if s.is_empty() {
        return true;
    }
    if re.is_match(s) {
        valid_braces(re.replace_all(s, "").to_string().as_str())
    } else {
        false
    }
}
