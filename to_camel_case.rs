use regex::{Captures, Regex};

fn to_camel_case(s: &str) -> String {
    let re = Regex::new(r"(?i)([^a-z]\w)").unwrap();
    return re
        .replace_all(s, |c: &Captures| {
            c.get(1).unwrap().as_str()[1..].to_uppercase()
        })
        .to_string();
}
