fn one_two_three(n: u32) -> [String; 2] {
    [
        format!(
            "{}{}",
            "9".repeat((n / 9) as usize),
            (if n > 0 && n % 9 == 0 {
                "".to_string()
            } else {
                (n % 9).to_string()
            })
        ),
        (if n == 0 { "0" } else { "1" }).repeat(n.max(1) as usize),
    ]
}
