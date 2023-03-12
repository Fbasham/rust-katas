fn smallest(n: i64) -> (i64, usize, usize) {
    let mut m = i64::MAX;
    let mut x = 0;
    let mut y = 0;
    let s = n.to_string();
    for i in 0..s.len() {
        let d = &s[i..i + 1];
        let t = format!("{}{}", &s[..i], &s[i + 1..]);
        for j in 0..t.len() + 1 {
            let w = format!("{}{}{}", &t[..j], d, &t[j..]);
            let n = w.parse::<i64>().unwrap();
            if n < m {
                m = n;
                x = i;
                y = j;
            }
        }
    }
    (m, x, y)
}
