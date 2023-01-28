fn find_digit(n: i32, k: i32) -> i32 {
    if k < 1 {
        return -1;
    }
    let s = format!("00000000000000000000000000000{}", n.abs());
    return s.chars().nth(s.len() - k as usize).unwrap() as i32 - 48;
}
