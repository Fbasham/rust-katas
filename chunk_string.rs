fn solution(s: &str) -> Vec<String> {
    let v = s.chars().collect::<Vec<_>>();
    v.chunks(2)
        .map(|e| match e.len() {
            2 => format!("{}{}", e[0], e[1]),
            _ => format!("{}_", e[0]),
        })
        .collect::<Vec<_>>()
}
