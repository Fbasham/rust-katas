use std::collections::HashSet;

fn compute_depth(n: u16) -> u8 {
    let mut s = HashSet::from([0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    let mut i = 1;
    while !s.is_empty() {
        let mut m = n * i;
        while m > 0 {
            let x = m % 10;
            if s.contains(&x) {
                s.remove(&x);
            }
            m /= 10;
        }
        i += 1;
    }
    (i - 1) as u8
}
