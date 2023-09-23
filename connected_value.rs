fn connected_values(a: &[Vec<u8>], n: u8, c: (usize, usize)) -> Vec<(usize, usize)> {
    let mut r = vec![];
    let mut q = if a[c.0][c.1] == n {
        vec![(c.0 as isize, c.1 as isize)]
    } else {
        vec![]
    };
    while !q.is_empty() {
        let (y, x) = q.pop().unwrap();
        if !r.contains(&(y, x)) {
            r.push((y, x))
        }
        for (dy, dx) in [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ] {
            let i = y + dy;
            let j = x + dx;
            if i >= 0
                && i < a.len() as isize
                && j >= 0
                && j < a[0].len() as isize
                && a[i as usize][j as usize] == n
                && !r.contains(&(i, j))
            {
                q.push((i, j));
            }
        }
    }
    r.iter().map(|t| (t.0 as usize, t.1 as usize)).collect()
}
