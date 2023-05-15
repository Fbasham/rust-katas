fn max_sum_dig(n: u32, d: u32) -> (usize, u32, u32) {
    let mut v = vec![];
    for i in 1000..=n {
        let s = i.to_string();
        if (0..=s.len() - 4).all(|x| {
            s[x..x + 4]
                .chars()
                .map(|e| e.to_digit(10).unwrap())
                .sum::<u32>()
                <= d
        }) {
            v.push(i);
        }
    }
    let u = v.iter().cloned().map(|e| e as f64).sum::<f64>() / (v.len() as f64);
    let x = v
        .iter()
        .cloned()
        .map(|e| e as f64)
        .min_by(|a, b| (a - u).abs().partial_cmp(&(b - u).abs()).unwrap())
        .unwrap();
    (v.len(), x as u32, v.iter().sum())
}
