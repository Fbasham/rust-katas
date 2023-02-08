fn solve(a: usize, b: usize) -> (usize, usize) {
    if a == 0 || b == 0 {
        (a, b)
    } else if a >= 2 * b {
        solve(a - 2 * b, b)
    } else if b >= 2 * a {
        solve(a, b - 2 * a)
    } else {
        (a, b)
    }
}
