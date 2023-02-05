fn extra_perfect(n: u32) -> Vec<u32> {
    (1..=n)
        .filter(|&e| e & 1 == 1 && format!("{e:b}").chars().nth(0).unwrap() == '1')
        .collect()
}
