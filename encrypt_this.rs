use regex::*;

fn encrypt_this(s: &str) -> String {
    Regex::new(r"\b(\w?)(\w?)(\w*?)(\w?)\b")
        .unwrap()
        .replace_all(s, |c: &Captures| {
            format!(
                "{}{}{}{}",
                c.get(1).unwrap().as_str().chars().nth(0).unwrap() as u32,
                c.get(4).unwrap().as_str(),
                c.get(3).unwrap().as_str(),
                c.get(2).unwrap().as_str(),
            )
        })
        .to_string()
}
