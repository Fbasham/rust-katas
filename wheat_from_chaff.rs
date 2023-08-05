fn wheat_from_chaff(xs: &[i32]) -> Vec<i32> {
    let mut v = xs.iter().cloned().collect::<Vec<_>>();
    let mut i = 0;
    let mut j = v.len() - 1;
    while i < j && i < xs.len() {
        if v[i] > 0 {
            while v[j] > 0 && j > i {
                j -= 1;
            }
            (v[i], v[j]) = (v[j], v[i]);
        }
        i += 1;
    }
    v
}
