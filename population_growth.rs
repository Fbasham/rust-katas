fn nb_year(p0: i32, i: f64, dp: i32, p: i32) -> i32 {
    let mut n = 0;
    let mut x = p0;
    while x < p {
        x = (((x as f64) * (1.0 + i / 100.0)) as i32) + dp;
        n += 1;
    }
    n
}
