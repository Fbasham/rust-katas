fn spiralize(n: usize) -> Vec<Vec<i8>> {
    let mut v = (0..n)
        .map(|_| (0..n).map(|_| 0).collect::<Vec<i8>>())
        .collect::<Vec<Vec<i8>>>();
    let d: Vec<(i8, i8)> = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut m = n;
    let mut y: i8 = 0;
    let mut x: i8 = 0;
    for k in 0..n {
        let dy = d[k % 4].0;
        let dx = d[k % 4].1;
        if (k + 1) < 5 && (k + 1) % 4 == 0 || (k + 1) > 4 && (k + 1) % 2 == 0 {
            m -= 2;
        }
        for _ in 0..m {
            v[y as usize][x as usize] = 1;
            (y, x) = (y + dy, x + dx);
        }
        (y, x) = (y - dy, x - dx);
    }
    v
}
