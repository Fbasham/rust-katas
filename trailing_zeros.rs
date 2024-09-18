fn trailing_zeros(n: i32) -> u32 {
    let s = format!("{n:b}");
    (s.len() - s.trim_end_matches("0").len()) as u32
}
