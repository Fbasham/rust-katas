use preloaded::ELEMENTS;

fn elemental_forms(word: &str) -> Vec<Vec<String>> {
    let s = word.to_lowercase();
    let mut q = vec![];
    let mut r = vec![];
    for e in ELEMENTS.keys() {
        if s.starts_with(&e.to_lowercase()) {
            q.push((&s[e.len()..s.len()], vec![e]));
        }
    }
    while q.len() > 0 {
        let (mut t, mut v) = q.pop().unwrap();
        if t.len() == 0 {
            r.push(
                v.iter()
                    .map(|e| format!("{} ({e})", ELEMENTS.get(**e).unwrap()))
                    .collect::<Vec<String>>(),
            );
            continue;
        }
        for e in ELEMENTS.keys() {
            if t.starts_with(&e.to_lowercase()) {
                let mut w = v.to_vec();
                w.push(e);
                q.push((&t[e.len()..t.len()], w));
            }
        }
    }
    r
}
