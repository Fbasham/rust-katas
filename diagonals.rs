fn diagonals(a: &[Vec<i8>]) -> Vec<Vec<i8>> {
    let N = a.len();
    let mut r = vec![];
    for j in 0..N {
        let mut t = vec![];
        let mut u = vec![];
        let mut v = vec![];
        let mut w = vec![];
        for k in 0..N - j {
            t.push(a[k][j + k]);
            u.push(a[j + k][k]);
            v.push(a[k][N - j - k - 1]);
            w.push(a[k + j][N - k - 1]);
        }
        r.push(t);
        if N > 1 {
            r.push(v)
        }
        if j > 0 {
            r.push(u);
            r.push(w);
        }
    }
    r
}
