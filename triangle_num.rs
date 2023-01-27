fn triangular(n: i32) -> i32 {
    if n < 0 {
        0
    } else {
        n * (n + 1) / 2
    }
}
