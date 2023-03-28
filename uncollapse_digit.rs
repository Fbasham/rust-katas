use regex::Regex;

fn uncollapse(s: &str) -> String {
    Regex::new(r"zero|one|two|three|four|five|six|seven|eight|nine").unwrap().find_iter(s).map(|e| e.as_str()).collect::<Vec<_>>().join(" ")
}