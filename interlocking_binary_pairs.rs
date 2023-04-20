fn interlockable(mut a: u64, mut b: u64) -> bool {
    while a > 0 && b > 0 {
        if a & 1 == 1 && b & 1 == 1 {
            return false;
        }
        a >>= 1;
        b >>= 1;
    }
    true
}
