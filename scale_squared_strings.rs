use regex::{Captures, Regex};

fn scale(s: &str, k: u32, n: u32) -> String {
    if (s.is_empty()) {
        return "".to_string();
    }
    let mut r: Vec<String> = vec![];
    for e in s.split("\n") {
        let t = Regex::new(r".").unwrap().replace_all(e, |c: &Captures| {
            c.get(0).unwrap().as_str().repeat(k as usize)
        });
        for _ in 0..n {
            r.push(t.clone().to_string());
        }
    }
    r.join("\n").to_string()
}
