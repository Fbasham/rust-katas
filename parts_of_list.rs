fn part_list(a: Vec<&str>) -> String {
    (1..a.len())
        .map(|i| format!("({}, {})", a[..i].join(" "), a[i..].join(" ")))
        .collect()
}
