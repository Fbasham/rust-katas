fn diving_minigame(a: &[i8]) -> bool {
    let mut b = 10;
    for e in a {
        if e < &0 {
            b -= 2;
            if b <= 0 {
                return false;
            }
        } else {
            b = (b + 4).min(10)
        }
    }
    true
}
