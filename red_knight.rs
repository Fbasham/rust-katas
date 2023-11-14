fn red_knight(n: u64, p: u64) -> (&'static str, u64) {
    let mut kx = 0;
    let mut ky = n;
    let mut px = p;
    while kx != px {
        kx += 2;
        px += 1;
        ky ^= 1;
    }
    (if ky == 0 { "White" } else { "Black" }, kx)
}
