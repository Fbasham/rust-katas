fn coin_combo(mut n: u64) -> [u64; 4] {
    let mut v = vec![];
    for e in [25, 10, 5, 1] {
        v.push(n / e);
        n %= e;
    }
    v.into_iter().rev().collect::<Vec<_>>().try_into().unwrap()
}
