fn find_missing(a: &[i32]) -> i32 {
    let mut v = a.to_vec();
    v.sort();
    let d = v.iter().zip(&v[1..]).map(|(i, j)| j - i).min().unwrap();
    for e in (v[0]..v[v.len() - 1]).step_by(d as usize) {
        if !v.contains(&e) {
            return e;
        }
    }
    0
}
