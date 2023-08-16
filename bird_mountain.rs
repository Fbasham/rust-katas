fn peak_height(m: &[&str]) -> u32 {
    let mut a = m
        .iter()
        .cloned()
        .map(|e| e.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let N = a.len() as isize;
    let M = a[0].len() as isize;
    let mut h = 0;
    while a.iter().any(|t| t.contains(&'^')) {
        let mut v = m
            .iter()
            .cloned()
            .map(|e| e.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        for i in 0..N {
            for j in 0..M {
                for (y, x) in [(i - 1, j), (i + 1, j), (i, j - 1), (i, j + 1)] {
                    if (i == 0 || i == N - 1 || j == 0 || j == M - 1)
                        || (y >= 0 && y < N && x >= 0 && x < M && a[y as usize][x as usize] == ' ')
                    {
                        v[i as usize][j as usize] = ' ';
                    }
                }
            }
        }
        a = v;
        h += 1;
    }
    h
}
