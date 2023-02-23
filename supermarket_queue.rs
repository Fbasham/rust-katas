fn queue_time(c: &[u32], n: u32) -> u32 {
    let mut a = vec![0; n as usize];
    let mut t = c.to_vec();
    let mut k = 0;
    while !t.is_empty() {
        for i in 0..a.len() {
            if a[i] == 0 && !t.is_empty() {
                a[i] = t.remove(0);
            }
        }
        let m = a.iter().cloned().min().unwrap();
        k += m;
        for i in 0..a.len() {
            a[i] -= m;
        }
    }
    k + a.iter().max().unwrap()
}
