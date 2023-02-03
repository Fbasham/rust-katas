fn word_value(a: &[&str]) -> Vec<i32> {
    let f = |s: &str| {
        s.replace(" ", "")
            .chars()
            .map(|e| (e as i32) - 96)
            .sum::<i32>()
    };
    a.iter()
        .enumerate()
        .map(|(i, s)| f(s) * ((i as i32) + 1))
        .collect()
}
