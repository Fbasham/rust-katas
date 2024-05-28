fn twos_difference(a: &[u32]) -> Vec<Vec<u32>> {
    let mut v = vec![];
    for i in 0..a.len() {
        for j in i + 1..a.len() {
            if (a[j] as i32 - a[i] as i32).abs() == 2 {
                v.push(if a[i] < a[j] {
                    vec![a[i], a[j]]
                } else {
                    vec![a[j], a[i]]
                })
            }
        }
    }
    v.sort();
    v
}
