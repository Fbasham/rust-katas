use std::collections::HashSet;

fn count_find_num(g: &[u16], k: u64) -> Option<(usize, u64)> {
    let p = g
        .iter()
        .fold(1_u64, |a, c| a.saturating_mul(c.clone() as u64));
    if p > k {
        return None;
    }
    let mut r = HashSet::new();
    r.insert(p);
    let mut m = p;
    for _ in 0..50 {
        for e in g {
            let mut u = HashSet::new();
            for t in &r {
                let n = t.saturating_mul(e.clone() as u64);
                if n <= k {
                    m = m.max(n);
                    u.insert(n);
                }
            }
            for v in u {
                r.insert(v);
            }
        }
    }
    return Some((r.len(), m));
}
