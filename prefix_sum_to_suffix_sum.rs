fn prefix_sums_to_suffix_sums(p: &[i32]) -> Vec<i32> {
    let a = (p
        .iter()
        .cloned()
        .take(1)
        .chain(p.iter().cloned().zip(&p[1..]).map(|(i, j)| j - i)))
    .collect::<Vec<_>>();
    let s = &mut p[p.len() - 1..].to_vec();
    for e in &a[..a.len() - 1] {
        s.push(s[s.len() - 1] - e)
    }
    s.to_owned()
}
