use itertools::Itertools;

fn rot(s: &str) -> String {
    s.split("\n")
        .map(|e| e.chars().rev().join(""))
        .collect::<Vec<_>>()
        .iter()
        .rev()
        .join("\n")
}

fn selfie_and_rot(s: &str) -> String {
    let n = s.split("\n").next().unwrap().len();
    let mut v = s
        .split("\n")
        .map(|e| format!("{}{}", e, ".".repeat(n)))
        .collect::<Vec<_>>();
    v.extend(
        rot(s)
            .split("\n")
            .map(|e| format!("{}{}", ".".repeat(n), e)),
    );
    v.join("\n")
}

fn oper(f: fn(&str) -> String, s: &str) -> String {
    f(s)
}
