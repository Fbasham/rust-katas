fn switcher(v: Vec<&str>) -> String {
    let s = "zyxwvutsrqponmlkjihgfedcba!? ";
    v.iter()
        .map(|e| s.chars().nth(e.parse::<usize>().unwrap() - 1).unwrap())
        .collect()
}
