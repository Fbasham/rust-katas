use itertools::Itertools;

fn diag_2_sym(s: &str) -> String {
    let v = s.split("\n").collect::<Vec<_>>();
    (0..v.len())
        .map(|i| {
            v.iter()
                .map(|e| e.chars().nth(v.len() - i - 1).unwrap())
                .rev()
                .join("")
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn rot_90_counter(s: &str) -> String {
    diag_2_sym(s)
        .split("\n")
        .map(|e| e.chars().rev().join(""))
        .join("\n")
}

fn selfie_diag2_counterclock(s: &str) -> String {
    let a = [s.to_string(), diag_2_sym(s), rot_90_counter(s)];
    (0..a[0].split("\n").count())
        .map(|i| a.iter().map(|e| e.split("\n").nth(i).unwrap()).join("|"))
        .join("\n")
}

fn oper(f: fn(&str) -> String, s: &str) -> String {
    f(s)
}
