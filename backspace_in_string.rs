use regex::Regex;

fn clean_string(s: &str) -> String {
    let mut t = s.to_string();
    let re = Regex::new(r"[^#]#").unwrap();
    while re.is_match(&t) {
        t = re.replace_all(&t, "").to_string();
    }
    t.replace("#", "")
}
