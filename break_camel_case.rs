use regex::*;

fn solution(s: &str) -> String {
    Regex::new(r"([A-Z])").unwrap().replace_all(s," $1").into_owned()
}s