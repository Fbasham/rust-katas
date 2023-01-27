fn find_multiples(n: u32, k: u32) -> Vec<u32> {
    let mut v = Vec::new();
    for i in 1..=k / n {
        v.push(i * n);
    }
    v
}
