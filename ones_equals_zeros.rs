use regex::Regex;

fn same_length(s: &str) -> bool {
    s.chars().filter(|&e| e == '1').count() == s.chars().filter(|&e| e == '0').count()
        && Regex::new("1+0+")
            .unwrap()
            .find_iter(s)
            .all(|m| m.as_str().matches("1").count() == m.as_str().matches("0").count())
}
