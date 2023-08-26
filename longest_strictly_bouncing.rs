fn longest_bouncy_list(a: &[i32]) -> Vec<i32> {
    let mut r = vec![];
    for i in 0..a.len() {
        let mut p = 0;
        let mut d = 0;
        for j in i..a.len() {
            if i == j {
                p = a[i];
                continue;
            }
            let n = a[j];
            if n == p || (n >= p && d == 1) || (n <= p && d == -1) {
                break;
            }
            if j - i + 1 > r.len() {
                r = a[i..j + 1].to_vec();
            }
            d = if n > p { 1 } else { -1 };
            p = a[j];
        }
    }
    if r.len() > 0 {
        r
    } else {
        a[..1].to_vec()
    }
}
