use regex::Regex;

fn find_text(s: &str, t: &str) -> String {
    let mut v = vec![];
    let re = Regex::new(format!(r"(?i)(\b{t}\b)").as_str()).unwrap();
    for m in Regex::new(r"[^.!?\n]+[.!?\n]").unwrap().find_iter(s) {
        let e = m.as_str().trim();
        if re.is_match(e) {
            v.push(format!(">{}", re.replace_all(e, "**$1**")));
        }
    }
    v.join("\n")
}
