fn bishop(start: &str, end: &str, k: u8) -> bool {
    let y1 = start.chars().nth(0).unwrap() as i32 - 97;
    let x1 = start.chars().nth(1).unwrap() as i32 - 48;
    let y2 = end.chars().nth(0).unwrap() as i32 - 97;
    let x2 = end.chars().nth(1).unwrap() as i32 - 48;
    if start == end {
        return true;
    }
    if (y1 % 2 == y2 % 2 && x1 % 2 != x2 % 2) || (y1 % 2 != y2 % 2 && x1 % 2 == x2 % 2) {
        return false;
    }
    if k == 1 {
        (y1 - y2).abs() / (if x1 == x2 { 1 } else { x1 - x2 }).abs() == 1
    } else {
        k > 1
    }
}
