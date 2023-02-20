fn sum_dig_pow(a: u64, b: u64) -> Vec<u64> {
    let mut v = vec![];
    let mut i = a;
    while i <= b {
        let x = i
            .to_string()
            .chars()
            .enumerate()
            .fold(0, |a, (i, c)| a + (c as u64 - 48).pow(i as u32 + 1));
        if i.to_string()
            .chars()
            .enumerate()
            .fold(0, |a, (i, c)| a + (c as u64 - 48).pow(i as u32 + 1))
            == i
        {
            v.push(i);
        }
        i += 1;
    }
    v
}
