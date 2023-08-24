fn make_chocolates(small: u32, big: u32, goal: u32) -> Option<u32> {
    let n = (goal / 5).min(big);
    for i in (0..=n).rev() {
        let s = (goal - i * 5) / 2;
        if 2 * s + 5 * i == goal && s <= small {
            return Some(s);
        }
    }
    None
}
