fn rule30(a: &[u8], n: u32) -> Vec<u8> {
    let mut v = a.to_vec();
    for _ in 0..n {
        v.insert(0, 0);
        v.insert(0, 0);
        v.push(0);
        v.push(0);
        v = (0..v.len() - 2)
            .map(|i| match (v[i], v[i + 1], v[i + 2]) {
                (0, 0, 0) => 0,
                (0, 0, 1) => 1,
                (0, 1, 0) => 1,
                (0, 1, 1) => 1,
                (1, 0, 0) => 1,
                (1, 0, 1) => 0,
                (1, 1, 0) => 0,
                _ => 0,
            })
            .collect::<Vec<_>>();
    }
    v
}
