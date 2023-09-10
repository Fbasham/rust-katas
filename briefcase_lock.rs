fn min_turns(s: &str, t: &str) -> u8 {
    s.chars().zip(t.chars()).map(|(i,j)| {
        let x = i as i8;
        let y = j as i8;
        (x-y).abs().min(10-(x-y).abs()) as u8
    }).sum()
}