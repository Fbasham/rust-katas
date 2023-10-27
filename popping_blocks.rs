fn pop_blocks(a: &[String]) -> Vec<String> {
    let mut v = a.to_vec();
    let mut i = 0;
    while i < v.len() {
        let mut j = i;
        while j < v.len() && v[j] == v[i] {
            j += 1;
        }
        if j - i > 1 {
            for _ in i..j {
                v.remove(i);
            }
            i = i.saturating_sub(1);
        } else {
            i += 1;
        }
    }
    v
}
