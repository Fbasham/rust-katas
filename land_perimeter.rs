pub fn land_perimeter(a: &[&str]) -> String {
    let mut p = 0;
    for i in 0..a.len() as i32 {
        for j in 0..a[0].len() as i32 {
            if a[i as usize].chars().nth(j as usize).unwrap() == 'X' {
                p += [(i - 1, j), (i + 1, j), (i, j - 1), (i, j + 1)]
                    .into_iter()
                    .fold(0, |r, (y, x)| {
                        r + (if y < 0
                            || y >= a.len() as i32
                            || x < 0
                            || x >= a[0].len() as i32
                            || a[y as usize].chars().nth(x as usize).unwrap() == 'O'
                        {
                            1
                        } else {
                            0
                        })
                    });
            }
        }
    }
    format!("Total land perimeter: {p}")
}
