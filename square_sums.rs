pub fn square_sums_row(n: u8) -> Option<Vec<u8>> {
    let a = (1..n + 1).map(|i| i).collect::<Vec<u8>>();
    let mut q = a
        .iter()
        .cloned()
        .map(|i| {
            (
                vec![i],
                a.iter().cloned().filter(|x| x != &i).collect::<Vec<u8>>(),
            )
        })
        .collect::<Vec<(Vec<u8>, Vec<u8>)>>();
    while q.len() > 0 {
        let (t, u) = q.pop().unwrap();
        if u.len() == 0 {
            return Some(t);
        }
        for i in &u {
            if ((t[t.len() - 1] + i) as f64).sqrt() % 1.0 == 0.0 {
                q.push((
                    t.iter().cloned().chain([*i]).collect(),
                    u.iter().cloned().filter(|x| x != i).collect(),
                ))
            }
        }
    }
    None
}
