fn break_chocolate(n: u32, m: u32) -> u64 {
    let x = n as u64;
    let y = m as u64;
    match x == 0 && y == 0 {
        true => 0,
        false => x * y - 1,
    }
}
