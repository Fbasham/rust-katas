use itertools::Itertools;

fn diag_1_sym(s: &str) -> String {
    s.split("\n")
        .next()
        .unwrap()
        .chars()
        .enumerate()
        .map(|(i, e)| s.split("\n").map(|e| e.chars().nth(i).unwrap()).join(""))
        .join("\n")
}
fn rot_90_clock(s: &str) -> String {
    diag_1_sym(s)
        .split("\n")
        .map(|e| e.chars().rev().join(""))
        .join("\n")
}
fn selfie_and_diag1(s: &str) -> String {
    let v = diag_1_sym(s);
    s.split("\n")
        .zip(v.split("\n"))
        .map(|(i, j)| format!("{i}|{j}"))
        .join("\n")
}
fn oper(f: impl Fn(&str) -> String, s: &str) -> String {
    f(s)
}
