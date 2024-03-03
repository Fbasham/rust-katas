fn partitions(n: u32) -> u128 {
    let mut v: Vec<u128> = vec![0; n as usize + 1];
    v[0] = 1;
    for i in 1..n as usize + 1 {
        for j in (i..=n as usize).rev() {
            v[j] += v[j - i];
        }
    }
    v[n as usize]
}
