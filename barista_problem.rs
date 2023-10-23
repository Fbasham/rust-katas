use itertools::Itertools;

fn barista(a: &[u8]) -> u16 {
    let mut t: u16 = 0;
    let mut r: u16 = 0;
    for e in a.iter().cloned().sorted() {
        t += e as u16 + (if t > 0 { 2 } else { 0 });
        r += t;
    }
    r as u16
}
