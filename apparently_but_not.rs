use fancy_regex::Regex;

fn apparently(s: &str) -> String {
    Regex::new(r"\b(and|but)\b(?! \bapparently\b)")
        .unwrap()
        .replace_all(s, "$1 apparently")
        .to_string()
}
