fn how_much(m: i32, n: i32) -> Vec<(String, String, String)> {
    let mut v = vec![];
    for i in m.min(n)..=n.max(m) {
        let b = (2 - i) / -7;
        let c = (1 - i) / -9;
        if i - 7 * b == 2 && i - 9 * c == 1 {
            v.push((
                format!("M: {}", i),
                format!("B: {}", b),
                format!("C: {}", c),
            ));
        }
    }
    v
}
