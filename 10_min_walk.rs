fn is_valid_walk(walk: &[char]) -> bool {
    let mut x = 0;
    let mut y = 0;
    for e in walk {
        match e {
            'e' => x += 1,
            'w' => x -= 1,
            'n' => y -= 1,
            _ => y += 1,
        }
    }
    x == 0 && y == 0 && walk.len() == 10
}
