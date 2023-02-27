fn meeting(s: &str) -> String {
    let mut v = s
        .split(";")
        .map(|e| {
            let x = e
                .to_uppercase()
                .split(":")
                .map(|e| e.to_string())
                .collect::<Vec<_>>();
            format!("({}, {})", &x[1], &x[0])
        })
        .collect::<Vec<_>>();
    v.sort();
    v.join("")
}
