fn solve(s: &str) -> String {
    let mut v = s.chars().filter(|&e| e.is_alphabetic()).collect::<Vec<_>>();
    let mut r = vec![];
    for e in s.chars() {
        r.push(match e.is_alphabetic() {
            true => v.pop().unwrap(),
            false => e,
        })
    }
    r.iter().collect()
}
