fn bingo<S: AsRef<str>>(a: &[(S, u8)], win: usize) -> &'static str {
    let n = a
        .iter()
        .map(|(s, c)| {
            if s.as_ref().chars().any(|e| e as u8 == *c) {
                1
            } else {
                0
            }
        })
        .sum::<usize>();
    if n >= win {
        "Winner!"
    } else {
        "Loser!"
    }
}
