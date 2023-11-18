fn build_matches_table(n: u32) -> Vec<Vec<(u32, u32)>> {
    let mut t = vec![
        (1..=n / 2).collect::<Vec<u32>>(),
        (n / 2 + 1..=n).collect::<Vec<u32>>(),
    ];
    let mut r = vec![];
    for _ in 1..n {
        let a = t[0].iter().cloned().zip(t[1].clone()).collect::<Vec<_>>();
        r.push(a);
        if n == 2 {
            break;
        }
        t = vec![
            vec![t[0][0], t[1][0]]
                .iter()
                .cloned()
                .chain(t[0][1..t[0].len() - 1].iter().cloned())
                .collect(),
            t[1][1..]
                .iter()
                .cloned()
                .chain(t[0][t[0].len() - 1..].iter().cloned())
                .collect(),
        ];
    }
    r
}
