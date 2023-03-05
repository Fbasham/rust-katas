use regex::Regex;

fn remove_parentheses(s: &str) -> String {
    match Regex::new(r"\([^\(\)]+\)|\(\)").unwrap().is_match(s) {
        true => remove_parentheses(
            Regex::new(r"\([^()]+\)|\(\)")
                .unwrap()
                .replace_all(s, "")
                .to_string()
                .as_str(),
        ),
        _ => s.to_string(),
    }
}
