fn pyramid_height(mut n: u32) -> u32 {
    let mut i = 1;
    while i * i < n && n > 0 {
        n -= i * i;
        if (i + 1) * (i + 1) > n {
            break;
        }
        i += 1
    }
    i
}
