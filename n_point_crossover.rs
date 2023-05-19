use itertools::Itertools;

pub fn crossover(ns: &[usize], xs: &[u8], ys: &[u8]) -> (Vec<u8>, Vec<u8>) {
    let mut v = ns.iter().cloned().unique().sorted().collect::<Vec<_>>();
    v.insert(0, 0);
    v.push(xs.len());
    let mut l: Vec<u8> = vec![];
    let mut r: Vec<u8> = vec![];
    for i in 0..v.len() - 1 {
        let x = &xs[v[i]..v[i + 1]].to_vec();
        let y = &ys[v[i]..v[i + 1]].to_vec();
        l.extend(if i % 2 == 0 { x } else { y });
        r.extend(if i % 2 == 0 { y } else { x });
    }
    (l, r)
}
