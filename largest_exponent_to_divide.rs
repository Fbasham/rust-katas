fn get_exponent(n: i32, p: u32) -> Option<u32> {
    for i in (0..=7).rev() {
        if p>1 && n%(p.pow(i as u32) as i32)==0 {
            return Some(i as u32)
        }
    }
    None
}