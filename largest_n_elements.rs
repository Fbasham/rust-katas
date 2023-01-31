fn largest(n: usize, a: &[i32]) -> Vec<i32> {
    let mut t = a.to_vec();
    t.sort();
    t[t.len() - n..].to_vec()
}
