fn stones_to_remove(s: &str) -> usize {
    let v = s.chars().collect::<Vec<_>>();
    v.iter().enumerate().fold(0, |a, c| {
        a + (if c.0 > 0 && &v[c.0 - 1] == c.1 { 1 } else { 0 })
    })
}
