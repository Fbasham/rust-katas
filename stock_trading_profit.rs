fn max_profit(a: &[u32]) -> u32 {
    let mut p = 0;
    let mut i = 0;
    while i < a.len() {
        let m = &a[i..].iter().max().unwrap();
        let j = &a[i..].iter().position(|e| e == *m).unwrap() + i;
        if i == j {
            i += 1;
            continue;
        }
        p += a[j] * ((j - i) as u32) - &a[i..j].iter().sum();
        i = j;
    }
    p
}
