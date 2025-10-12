use regex::{Captures, Regex};

fn generate_hashtag(s: &str) -> Option<String> {
    let r = format!(
        "#{}",
        Regex::new(r"^\S| \S")
            .unwrap()
            .replace_all(&s.trim().to_lowercase(), |c: &Captures| {
                c.get(0).unwrap().as_str().trim().to_uppercase()
            })
            .to_string()
    )
    .replace(" ", "");
    if r.len() < 2 || r.len() > 140 {
        None
    } else {
        Some(r)
    }
}
