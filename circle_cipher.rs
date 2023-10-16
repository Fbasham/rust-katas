fn encode(s: String) -> String {
    let mut a = s.chars().collect::<Vec<_>>();
    let mut r = String::new();
    while a.len() > 0 {
        r += &a.remove(0).to_string();
        if a.len() > 0 {
            r += &a.pop().unwrap().to_string();
        }
    }
    r
}

fn decode(s: String) -> String {
    (0..s.len())
        .step_by(2)
        .map(|i| s.chars().nth(i).unwrap())
        .collect::<String>()
        + &(1..s.len())
            .step_by(2)
            .map(|i| s.chars().nth(i).unwrap())
            .rev()
            .collect::<String>()
}
