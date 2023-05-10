fn expanded_form(n: f64) -> String {
    let s = n.to_string();
    let mut v = vec![];
    for (i, e) in s.split(".").next().unwrap().chars().rev().enumerate() {
        if e != '0' {
            v.insert(0, e.to_string() + &"0".repeat(i));
        }
    }
    for (i, e) in s
        .split(".")
        .skip(1)
        .next()
        .unwrap_or("")
        .chars()
        .enumerate()
    {
        if e != '0' {
            v.push(format!("{}/1{}", e, "0".repeat(i + 1)));
        }
    }
    v.join(" + ")
}
