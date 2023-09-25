use std::collections::*;

fn finish_all(a: &[(u32, u32)]) -> bool {
    let mut d = HashMap::new();
    for (i, j) in a {
        d.entry(i).or_insert(HashSet::new()).insert(j);
    }
    for (k, v) in &d {
        let mut q = v.iter().collect::<Vec<_>>();
        let mut s = HashSet::new();
        while q.len() > 0 {
            let e = q.pop().unwrap();
            s.insert(e);
            if e == k {
                return false;
            }
            if !d.contains_key(e) {
                continue;
            }
            for x in d.get(e).unwrap() {
                if !s.contains(x) {
                    q.push(x);
                }
            }
        }
    }
    true
}
