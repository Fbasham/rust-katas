fn x(n: u32) -> String {
    let mut v = vec![];
    for x in 0..=n / 2 {
        let i = x as usize;
        v.push(if x == n / 2 {
            format!("{}■{}", "□".repeat(i), "□".repeat(i))
        } else {
            format!(
                "{}■{}■{}",
                "□".repeat(i),
                "□".repeat((n - 2 - 2 * x) as usize),
                "□".repeat(i)
            )
        });
    }
    v.extend(v.iter().cloned().rev().skip(1).collect::<Vec<_>>());
    v.join("\n")
}
