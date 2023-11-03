fn riders(a: &Vec<u32>) -> u32 {
    let mut c = 1;
    let mut d = 0;
    for e in a {
        if d+e>100 {
            c += 1;
            d = *e;
        }
        else {d += *e}
    }
    c
}