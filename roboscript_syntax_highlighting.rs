use regex::Regex;

pub fn highlight(s: &str) -> String {
    let mut r = String::new();
    for m in Regex::new("F+|L+|R+|\\d+|.").unwrap().find_iter(s) {
        let e = m.as_str();
        let c = e.chars().next().unwrap();
        r = format!(
            "{}{}",
            r,
            match c {
                '(' => e.to_string(),
                ')' => e.to_string(),
                'F' => format!("<span style=\"color: pink\">{}</span>", e),
                'L' => format!("<span style=\"color: red\">{}</span>", e),
                'R' => format!("<span style=\"color: green\">{}</span>", e),
                _ => format!("<span style=\"color: orange\">{}</span>", e),
            }
        );
    }
    r
}
