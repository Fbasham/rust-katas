use itertools::Itertools;

fn hist(s: &str) -> String {
    let mut v = vec![("u", 0), ("w", 0), ("x", 0), ("z", 0)];
    for e in s.chars() {
        match e {
            'u' => v[0].1 += 1,
            'w' => v[1].1 += 1,
            'x' => v[2].1 += 1,
            'z' => v[3].1 += 1,
            _ => (),
        }
    }
    v.iter()
        .filter(|&t| t.1 > 0)
        .map(|t| format!("{}  {}     {}", t.0, t.1, "*".repeat(t.1)))
        .join("\r")
}
