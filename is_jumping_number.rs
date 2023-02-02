fn jumping_number(n: u64) -> String {
    let s = n.to_string().chars().map(|e| e as i8).collect::<Vec<i8>>();
    match s.windows(2).all(|e| (e[0] - e[1]).abs() == 1) {
        true => "Jumping!!".to_string(),
        false => "Not!!".to_string(),
    }
}
