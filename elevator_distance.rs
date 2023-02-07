fn elevator_distance(a: &[i16]) -> i16 {
    let mut d = 0;
    for e in a.windows(2) {
        d += (e[0] - e[1]).abs()
    }
    d
}
