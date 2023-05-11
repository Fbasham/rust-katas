fn the_janitor(s: &str) -> [usize; 26] {
    let mut a = [0; 26];
    for e in 'a'..='z' {
        if s.contains(e) {
            let i = s.chars().position(|x| e == x).unwrap();
            let j = s.len() - s.chars().rev().position(|x| e == x).unwrap();
            a[(e as usize) - 97] = j - i
        }
    }
    a
}
