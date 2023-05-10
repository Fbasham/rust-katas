fn mirror(s: &str) -> String {
    let a = s.split(" ").collect::<Vec<_>>();
    let n = a.iter().map(|e| e.len()).max().unwrap() + 4;
    let mut v = vec!["*".repeat(n)];
    for e in a {
        v.push(format!(
            "* {}{}*",
            e.chars().rev().collect::<String>(),
            " ".repeat(n - e.len() - 3)
        ));
    }
    v.push("*".repeat(n));
    v.join("\n")
}
