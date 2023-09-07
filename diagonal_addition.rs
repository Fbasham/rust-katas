fn f(a: &[Vec<i8>], mut y: isize, mut x: isize, dy: isize, dx: isize) -> i128 {
    let mut s = 1 as i128;
    while y >= 0 && y < a.len() as isize && x >= 0 && x < a[0].len() as isize {
        s *= a[y as usize][x as usize] as i128;
        y += dy;
        x += dx;
    }
    s as i128
}

fn sum_prod_diags(a: &[Vec<i8>]) -> i128 {
    let mut r = 0;
    for x in 0..a.len() as isize {
        r += f(a, 0, x, 1, 1)
    }
    for y in 1..a.len() as isize {
        r += f(a, y, 0, 1, 1)
    }
    for y in 0..a.len() as isize {
        r -= f(a, y, 0, -1, 1)
    }
    for x in 1..a[0].len() as isize {
        r -= f(a, a[0].len() as isize - 1, x, -1, 1)
    }
    r
}
