fn sum_pow_dig_seq(start: u32, p: u32, k: u32) -> (u32, Vec<u32>, u32, u32) {
    let mut n = start;
    let mut a = vec![];
    let mut i = 0;
    while i<k && !a.contains(&n) {
        a.push(n);
        n = n.to_string().chars().map(|e| e.to_digit(10).unwrap().pow(p)).sum();
        i += 1;
    }
    let x = a.iter().position(|&e| e==n).unwrap();
    let t = &a[x as usize..];
    (
        x as u32,t.to_vec(),t.len() as u32,
        if (k as usize)<a.len() {a[k as usize]} else {t[(k-x as u32) as usize%t.len()]}
    )
}