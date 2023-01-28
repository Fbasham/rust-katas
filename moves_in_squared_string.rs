fn hor_mirror(s: String) -> String {
    s.split('\n').rev().collect::<Vec<&str>>().join("\n")
}

fn vert_mirror(s: String) -> String {
    s.split('\n')
        .map(|e| e.chars().rev().collect::<String>())
        .collect::<Vec<String>>()
        .join("\n")
}

fn oper(f: fn(String) -> String, s: String) -> String {
    f(s)
}
