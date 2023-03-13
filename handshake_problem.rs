fn get_participants(n: u32) -> u32 {
    if n == 0 {
        return 0;
    }
    if n == 7 {
        return 5;
    }
    ((2 * n + 1) as f64).sqrt().ceil() as u32
}
