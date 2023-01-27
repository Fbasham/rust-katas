fn open_or_senior(a: Vec<(i32, i32)>) -> Vec<String> {
    a.iter().map(|e| (if e.0>54 && e.1>7 {"Senior"} else {"Open"}).to_string()).collect()
}