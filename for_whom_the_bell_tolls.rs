fn bell(n: u32) -> Vec<u32> {
    let mut v = vec![];
    for i in 0..n / 2 + (if n % 2 == 1 { 1 } else { 0 }) {
        v.push(if v.len() == 0 {
            n
        } else {
            v[v.len() - 1] + (n - 2 * i)
        });
    }
    let t = v.iter().cloned().rev().collect::<Vec<_>>();
    v.extend(if n % 2 == 1 { &t[1..] } else { &t });
    v
}
