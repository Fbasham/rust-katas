fn postfix_evaluator(s: &str) -> i64 {
    let mut t = vec![];
    for e in s.split(" ") {
        if let Ok(v) = e.parse::<i64>() {
            t.push(v);
        } else {
            let y = t.pop().unwrap();
            let x = t.pop().unwrap();
            match e {
                "+" => t.push(x + y),
                "-" => t.push(x - y),
                "*" => t.push(x * y),
                _ => t.push(x / y),
            }
        }
    }
    t[0]
}
