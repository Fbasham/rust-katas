use regex::Regex;

fn change(s: &str, prog: &str, version: &str) -> String {
    let rev = Regex::new(r"Version: \d+\.\d+\n").unwrap();
    let rev2 = Regex::new(r"Version: 2.0\n").unwrap();
    let rep = Regex::new(r"Phone: \+1\-\d{3}\-\d{3}\-\d{4}\n").unwrap();
    if !rev.is_match(s) || !rep.is_match(s) {
        return "ERROR: VERSION or PHONE".to_string();
    }
    format!(
        "Program: {} Author: g964 Phone: {} Date: 2019-01-01 Version: {}",
        prog,
        "+1-503-555-0090",
        if rev2.is_match(s) { "2.0" } else { version }
    )
}
