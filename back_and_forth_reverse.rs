fn arrange(s: &[i32]) -> Vec<i32> {
    let mut v = vec![];
    if s.len() < 1 {
        return v;
    }
    let k = s.len();
    for i in 0..=(k / 2 - (if k % 2 == 0 { 1 } else { 0 })) {
        let j = k - i - 1;
        if i % 2 == 0 {
            v.push(s[i]);
            if i != j {
                v.push(s[k - i - 1]);
            }
        } else {
            v.push(s[k - i - 1]);
            if i != j {
                v.push(s[i]);
            }
        }
    }
    v
}
