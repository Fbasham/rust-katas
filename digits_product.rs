fn digits_product(n: u32) -> Option<u32> {
    if n < 10 {
        return Some(10 + n);
    }
    let mut v = (2..=9).rev().collect::<Vec<_>>();
    while v.len() > 0 {
        let m = v.pop().unwrap();
        let t = m.to_string().chars().fold(1, |a, c| a * (c as u32 - 48));
        if t == n {
            return Some(m);
        }
        if t > n {
            continue;
        }
        for i in 2..=9 {
            v.insert(0, m * 10 + i)
        }
    }
    None
}
