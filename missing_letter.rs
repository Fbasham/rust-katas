fn find_missing_letter(a: &[char]) -> char {
    let v = a.to_vec();
    for e in a[0]..a[a.len() - 1] {
        if !v.contains(&e) {
            return e;
        };
    }
    return ' ';
}
