use regex::{Captures, Regex};

fn reverse_words(s: &str) -> String {
    let re = Regex::new(r"[\w\.]+").unwrap();
    re.replace_all(s, |c: &Captures| c[0].chars().rev().collect::<String>())
        .to_string()
}
