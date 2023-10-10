fn luxhouse(a: &[u32]) -> Vec<u32> {
    a.iter()
        .enumerate()
        .map(|(i, e)| {
            let m = a.iter().skip(i + 1).max().unwrap_or(&0);
            if m < e {
                0
            } else {
                m - e + 1
            }
        })
        .collect()
}
