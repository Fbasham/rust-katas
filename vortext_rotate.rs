fn rotate_like_a_vortex(a: &[Vec<u8>]) -> Vec<Vec<u8>> {
    let mut v = a.iter().cloned().collect::<Vec<_>>();
    for i in 0..v.len() / 2 {
        let t = v[i..v.len() - i]
            .iter()
            .cloned()
            .map(|x| x[i..x.len() - i].iter().cloned().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let w = (0..t.len())
            .map(|x| t.iter().map(|u| u[x]).collect::<Vec<_>>())
            .rev()
            .collect::<Vec<_>>();
        for y in 0..w.len() {
            for x in 0..w.len() {
                v[y + i][x + i] = w[y][x];
            }
        }
    }
    v
}
