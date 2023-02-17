use regex::Regex;

fn spoonerize(s: &str) -> String {
    Regex::new(r"(\w)(\w+ )(\w)(\w+)").unwrap().replace(s,"$3$2$1$4").to_string()
}