fn shared_bits(mut a: u32, mut b: u32) -> bool {
    let mut c = 0;
    while a > 0 && b > 0 {
        if a & 1 == 1 && b & 1 == 1 {
            c += 1
        }
        (a, b) = (a >> 1, b >> 1);
    }
    c > 1
}
