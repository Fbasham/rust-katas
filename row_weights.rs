fn row_weights(v: Vec<u32>) -> (u32, u32) {
    let a = v
        .iter()
        .enumerate()
        .fold(0, |x, (i, &e)| x + (if i % 2 == 0 { e } else { 0 }));
    let b = v
        .iter()
        .enumerate()
        .fold(0, |x, (i, &e)| x + (if i % 2 == 1 { e } else { 0 }));
    (a, b)
}
