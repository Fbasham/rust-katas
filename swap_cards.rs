fn swap_cards(n1: u8, n2: u8) -> bool {
    let a = n1.to_string().chars().nth(0).unwrap();
    let b = n1.to_string().chars().nth(1).unwrap();
    let c = n2.to_string().chars().nth(0).unwrap();
    let d = n2.to_string().chars().nth(1).unwrap();
    match a <= b {
        true => format!("{c}{b}").parse::<u8>().unwrap() > format!("{a}{d}").parse::<u8>().unwrap(),
        _ => format!("{a}{c}").parse::<u8>().unwrap() > format!("{b}{d}").parse::<u8>().unwrap(),
    }
}
