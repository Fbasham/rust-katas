fn zoom(n: i32) -> String {
    let mut v = vec!["■".to_string()];
    for i in 0..n as usize / 2 {
        let e = if i % 2 == 0 { "□" } else { "■" };
        let x = e.repeat(2 * i + 3);
        v = v.iter().map(|t| format!("{e}{t}{e}")).collect();
        v.insert(0, x.to_string());
        v.push(x.to_string());
    }
    v.join("\n")
}
