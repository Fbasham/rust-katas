fn magic_square(n: u32) -> Vec<Vec<u32>> {
    let mut v = (0..n)
        .map(|_| (0..n).map(|_| 0).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut y = 0 as i32;
    let mut x = n as i32 >> 1;
    for k in 1..=n * n {
        v[y as usize][x as usize] = k;
        if k == n * n {
            break;
        }
        let mut ty = y - 1;
        let mut tx = x + 1;
        if ty < 0 {
            ty = n as i32 - 1;
        }
        if tx >= n as i32 {
            tx = 0
        }
        if v[ty as usize][tx as usize] != 0 {
            ty = y;
            tx = x;
            while v[ty as usize][tx as usize] != 0 {
                ty += 1;
            }
        }
        y = ty;
        x = tx;
    }
    v
}
