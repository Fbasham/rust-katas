fn min_umbrellas(a: &[&str]) -> usize {
    let mut h = 0;
    let mut o = 0;
    for (i, e) in a.iter().enumerate() {
        if e == &"rainy" || e == &"thunderstorms" {
            if i % 2 == 0 && h == 0 {
                h += 1
            }
            if i % 2 == 1 && o == 0 {
                o += 1
            }
            if i % 2 == 0 {
                h -= 1;
                o += 1;
            } else {
                o -= 1;
                h += 1;
            }
        }
    }
    h + o
}
