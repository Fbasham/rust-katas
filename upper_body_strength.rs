fn alex_mistakes(n: u32, t: u32) -> u32 {
    let mut x = t - n * 6;
    let mut k = 5;
    let mut i = 0;
    while x > 0 {
        if k > x {
            break;
        }
        x -= k;
        k *= 2;
        i += 1;
    }
    i
}
