fn faulty_odometer(n: usize) -> usize {
    usize::from_str_radix(
        &n.to_string()
            .chars()
            .map(|e| match e > '4' {
                true => (e as u8 - 1) as char,
                _ => e,
            })
            .collect::<String>(),
        9,
    )
    .unwrap()
}
