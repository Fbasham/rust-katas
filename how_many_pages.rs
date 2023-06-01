fn amount_of_pages(mut n: u32) -> u32 {
    let mut k = 0;
    let mut p = 0;
    while n > 0 {
        let d = 9 * 10_u32.pow(k);
        if d * (k + 1) >= n {
            p += n / (k + 1);
            break;
        }
        p += d;
        n -= d * (k + 1);
        k += 1;
    }
    p
}
