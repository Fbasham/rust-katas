fn page_digits(mut n: u64) -> u64 {
    let mut p = 0;
    let mut i = 0;
    while n > 0 {
        let x = n.min(9 * 10_u64.pow(i));
        p += x * (i as u64 + 1);
        n -= x;
        i += 1;
    }
    p
}
