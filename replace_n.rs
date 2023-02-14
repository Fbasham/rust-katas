use itertools::Itertools;

fn solve(s: &str, k: usize) -> String {
    let mut t = s.to_string();
    let v = s.chars().sorted().collect::<Vec<_>>();
    for e in &v[..k.min(t.len())] {
        t = t.replacen(*e, "", 1);
    }
    t
}
