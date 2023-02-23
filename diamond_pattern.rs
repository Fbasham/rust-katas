fn print(n: i32) -> Option<String> {
    if n < 1 || n % 2 == 0 {
        return None;
    }
    let mut v = vec![];
    for i in (1..=n).step_by(2) {
        v.push(" ".repeat(((n - i) / 2) as usize) + "*".repeat(i as usize).as_str());
    }
    Some(
        v.iter()
            .chain(v.iter().rev().skip(1))
            .map(|e| e.clone())
            .collect::<Vec<_>>()
            .join("\n")
            + "\n",
    )
}
