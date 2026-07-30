fn count_min_rotations(a: &[u8]) -> u32 {
    (1..7).map(|m| a.iter().map(|k| if *k==m {0} else if *k==7-m {2} else {1}).sum()).min().unwrap()
}