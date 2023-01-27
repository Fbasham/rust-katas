fn xo(s: &'static str) -> bool {
    let x = s.to_lowercase().chars().filter(|&e| e == 'x').count();
    let o = s.to_lowercase().chars().filter(|&e| e == 'o').count();
    x == o
}
