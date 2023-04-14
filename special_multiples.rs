fn count_spec_mult(k: u8, m: u64) -> u64 {
    let a = vec![
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71,
    ];
    let p = a[..k as usize].iter().product::<u64>();
    m / p
}
