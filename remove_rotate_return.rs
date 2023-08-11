fn rotate_and_remove(a: &[Vec<u8>]) -> u8 {
    let mut v = a
        .iter()
        .cloned()
        .map(|t| t.iter().cloned().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    while v.len() > 1 {
        v = (0..v[0].len())
            .map(|i| v.iter().cloned().map(|t| t[t.len() - i - 1]).collect())
            .collect();
        v = v
            .iter()
            .cloned()
            .map(|t| {
                let n = t.iter().min().unwrap();
                let m = t.iter().max().unwrap();
                let mut x = 0;
                let mut y = 0;
                t.iter()
                    .filter(|e| {
                        if e == &n && x == 0 {
                            x = 1;
                            return false;
                        }
                        if e == &m && y == 0 {
                            y = 1;
                            return false;
                        }
                        true
                    })
                    .cloned()
                    .collect()
            })
            .collect();
    }
    v[0][0]
}
