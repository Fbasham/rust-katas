fn prescribe(d: u16, a: u16, b: u16) -> u16 {
    let mut m = 0;
    let x = a.min(b);
    let y = a.max(b);
    for i in (0..=d / x).rev() {
        let j = (d - i * a) / b;
        m = m.max(i * a + j * b);
    }
    m
}
