fn delete_digit(n: u32) -> u32 {
    let s = n.to_string();
    (0..s.len())
        .map(|i| (s[0..i].to_string() + &s[i + 1..]).parse::<u32>().unwrap())
        .max()
        .unwrap()
}
