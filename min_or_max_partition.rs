fn find_spec_partition(n: u32, k: u32, s: &str) -> Vec<u32> {
    match s {
        "min" => (0..k).map(|i| if i == 0 { n - k + 1 } else { 1 }).collect(),
        _ => {
            let mut v = vec![0; k as usize];
            for i in 0..n {
                v[(i % k) as usize] += 1;
            }
            v
        }
    }
}
