fn seven(mut n: i64) -> (i64, i32) {
    let mut c = 0;
    while n > 99 {
        n = n / 10 - 2 * (n % 10);
        c += 1;
    }
    (n, c)
}
