fn cockroaches(maze: &[&str]) -> [u32; 10] {
    let a = maze
        .iter()
        .map(|t| t.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut r = [0; 10];
    let mut q = vec![];
    for i in 1..a.len() - 1 {
        for j in 1..a[0].len() - 1 {
            if a[i][j] != ' ' {
                q.push((i as isize, j as isize, a[i][j]))
            }
        }
    }
    while q.len() > 0 {
        let (mut y, mut x, mut k) = q.pop().unwrap();
        let (dy, dx) = match k {
            'U' => (-1, 0),
            'L' => (0, -1),
            'D' => (1, 0),
            _ => (0, 1),
        };
        (y, x) = (y + dy, x + dx);
        if y < 0 || y >= a.len() as isize || x < 0 || x >= a[0].len() as isize {
            y = y.max(0).min(a.len() as isize - 1);
            x = x.max(0).min(a[0].len() as isize - 1);
            k = match k {
                'U' => 'L',
                'L' => 'D',
                'D' => 'R',
                _ => 'U',
            };
        }
        if a[y as usize][x as usize].is_ascii_digit() {
            r[a[y as usize][x as usize] as usize - 48] += 1;
        } else {
            q.push((y, x, k));
        }
    }
    r
}
