fn thirt(n: i64) -> i64 {
    let v = vec![1, 10, 9, 12, 3, 4];
    let mut p = 0;
    let mut c = n;
    while c != p {
        p = c;
        c = c
            .to_string()
            .chars()
            .rev()
            .enumerate()
            .map(|(i, e)| (e as i64 - 48) * v[i % v.len()])
            .sum();
    }
    c
}
