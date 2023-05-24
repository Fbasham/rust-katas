fn snail(a: &[Vec<i32>]) -> Vec<i32> {
    let mut v = vec![];
    let mut m = a
        .iter()
        .map(|t| t.iter().map(|e| *e).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    while !m.is_empty() {
        v.extend(m.remove(0));
        if m.len() > 0 {
            m = (0..m[0].len())
                .map(|i| m.iter().map(|t| t[t.len() - i - 1]).collect())
                .collect();
        }
    }
    v
}
