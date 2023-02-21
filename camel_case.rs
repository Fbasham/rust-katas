use regex::{Captures, Regex};

fn camel_case(s: &str) -> String {
    Regex::new(r"( |^)(.)")
        .unwrap()
        .replace_all(s, |c: &Captures| {
            c.get(0).unwrap().as_str().trim().to_uppercase()
        })
        .trim()
        .to_string()
}
