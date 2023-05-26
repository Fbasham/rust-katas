fn find_int(a: u32, b: u32, k: u32) -> (usize, Vec<u32>) {
    let mut v = vec![];
    for i in a..=b {
        let m = i
            .to_string()
            .chars()
            .enumerate()
            .map(|(i, e)| ((i + 1) as u32) * e.to_digit(10).unwrap())
            .sum();
        let n: u32 = i
            .to_string()
            .chars()
            .enumerate()
            .map(|(i, e)| ((i + 1) as u32).pow(e.to_digit(10).unwrap()))
            .sum();
        let s: u32 = (1..=m).filter(|&i| m % i == 0).sum();
        if k * n % s == 0 {
            v.push(i);
        }
    }
    (v.len(), v)
}
