fn square_digits_sequence(mut n: u32) -> usize {
    let mut v = vec![];
    while !v.contains(&n) {
        v.push(n);
        n = n
            .to_string()
            .chars()
            .fold(0, |a, c| a + c.to_digit(10).unwrap().pow(2));
    }
    v.len() + 1
}
