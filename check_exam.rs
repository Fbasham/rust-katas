fn check_exam(a: &[&str], b: &[&str]) -> i64 {
    b.iter()
        .enumerate()
        .fold(0, |x, (i, &e)| {
            x + (if e == "" {
                0
            } else if e == a[i] {
                4
            } else {
                -1
            })
        })
        .max(0)
}
