fn merge(a: &[u8]) -> Vec<u8> {
    let mut v = vec![];
    let t = a.iter().cloned().filter(|&e| e != 0).collect::<Vec<_>>();
    for i in 0..t.len() {
        let e = t[i];
        if v.len() > 0 && v[v.len() - 1] == e && t[i - 1] == e {
            v.pop();
            v.push(e + e);
        } else {
            v.push(e)
        }
    }
    v.extend(vec![0; a.len() - v.len()]);
    v
}
