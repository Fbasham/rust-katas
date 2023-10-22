fn find_subarr_maxsum(a: &[i32]) -> (Vec<Vec<i32>>, i32) {
    let mut t = vec![0; a.len()];
    for i in 0..a.len() {
        t[i] = if i == 0 { a[i] } else { t[i - 1] + a[i] };
    }
    t.insert(0, 0);
    let mut m = 0;
    let mut r = vec![];
    for i in 0..a.len() {
        for j in i + 1..=a.len() {
            let s = t[j] - t[i];
            if s == m {
                r.push(a[i..j].to_vec());
            }
            if s > m {
                m = s;
                r = vec![a[i..j].to_vec()];
            }
        }
    }
    (r, m)
}
