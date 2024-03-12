fn count_checkerboard(w: u128, h: u128, r: u128) -> u128 {
    let cols = w/r/2;
    let rows = h/r/2;
    let ew = r*cols+(w.saturating_sub(2*r*cols).saturating_sub(r));
    let eh = r*rows+(h.saturating_sub(2*r*rows).saturating_sub(r));
    (w-ew)*eh+(h-eh)*ew
}