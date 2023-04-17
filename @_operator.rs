fn evaluate(s: String) -> Option<i64> {
    let v = s
        .split(" @ ")
        .map(|e| e.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    if v[0] != 0 && v.contains(&0) {
        return None;
    }
    Some(
        v.iter()
            .cloned()
            .reduce(|a, b| a + b + a - b + a * b + a / b)
            .unwrap(),
    )
}
