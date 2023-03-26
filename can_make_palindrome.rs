fn f(s: String) -> bool {
    s.chars().rev().collect::<String>() == s
}

fn solve(s: &str) -> String {
    if f(s.to_string()) {
        return "OK".to_string();
    }
    match (0..s.len()).any(|i| f(s[..i].to_owned() + &s[i + 1..])) {
        true => "remove one".to_string(),
        _ => "not possible".to_string(),
    }
}
