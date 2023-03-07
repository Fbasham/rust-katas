fn int32_to_ip(mut n: u32) -> String {
    let mut v = vec![];
    while n > 0 || v.len() < 4 {
        v.push((n & 255).to_string());
        n >>= 8;
    }
    v.into_iter().rev().collect::<Vec<String>>().join(".")
}
