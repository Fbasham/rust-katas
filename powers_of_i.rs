fn pofi(n: u32) -> &'static str {
    match n % 4 {
        0 => "1",
        1 => "i",
        2 => "-1",
        _ => "-i",
    }
}
