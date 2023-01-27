fn accum(s: &str) -> String {
    let mut v: Vec<String> = vec![];
    for (i, e) in s.chars().enumerate() {
        v.push(format!(
            "{}{}",
            e.to_uppercase(),
            e.to_string().to_lowercase().repeat(i)
        ))
    }
    v.join("-")
}
