fn race_podium(n: u32) -> (u32, u32, u32) {
    let mut i = (n as f64 / 3.0).ceil() as u32;
    let j = i + 1;
    let mut k = n - i - i - 1;
    if k == 0 {
        i -= 1;
        k = 1;
    }
    (i, j, k)
}
