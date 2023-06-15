fn solve(a: &[u32; 3]) -> u32 {
    let mut v = a.iter().map(|e| *e).collect::<Vec<_>>();
    let mut c = 0;
    while v.iter().any(|&e| e > 0) {
        let min = v.iter().cloned().min().unwrap();
        let i = v.iter().cloned().position(|e| e == min).unwrap();
        v[i] -= 1;
        let max = v.iter().cloned().max().unwrap();
        let j = v.iter().cloned().position(|e| e == max).unwrap();
        if max == 0 || i == j {
            break;
        }
        v[j] -= 1;
        c += 1;
        v = v.iter().filter(|&e| e > &0).map(|e| *e).collect();
    }
    c
}
