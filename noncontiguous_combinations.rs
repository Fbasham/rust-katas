fn find_comb_noncontig(a: &[i32], n: i32, k: u32) -> u32 {
    let mut q = vec![(a.iter().cloned().collect::<Vec<i32>>(), vec![])];
    let mut c = 0;
    while q.len() > 0 {
        let (t, r): (Vec<i32>, Vec<i32>) = q.pop().unwrap();
        for i in 0..t.len() {
            let mut u = r.iter().cloned().collect::<Vec<i32>>();
            u.push(t[i]);
            let s = u.iter().sum::<i32>();
            if s >= n - k as i32 && s <= n + k as i32 {
                c += 1;
            }
            if i + 2 <= t.len() {
                q.push((t[i + 2..].iter().cloned().collect(), u));
            }
        }
    }
    c
}
