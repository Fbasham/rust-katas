fn disarium_number(n: u32) -> String {
    match n
        .to_string()
        .chars()
        .enumerate()
        .map(|(i, e)| (e as u32 - 48).pow(i as u32 + 1))
        .sum::<u32>()
        == n
    {
        true => "Disarium !!".to_string(),
        false => "Not !!".to_string(),
    }
}
