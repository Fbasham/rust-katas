fn game(n: u64) -> Vec<u64> {
    let x = n * n;
    match x % 2 == 0 {
        true => vec![x / 2],
        _ => vec![x, 2],
    }
}
