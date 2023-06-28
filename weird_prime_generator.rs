fn gcd(a: i64, b: i64) -> i64 {
    if b > 0 {
        gcd(b, a % b)
    } else {
        a
    }
}

fn A(n: i64) -> Vec<i64> {
    let mut v = vec![];
    for i in 1..n {
        if i == 1 {
            v.push(7)
        } else {
            v.push(v[v.len() - 1] + gcd(i, v[v.len() - 1]))
        }
    }
    v
}

fn G(n: i64) -> Vec<i64> {
    let a = A(n + 1);
    let mut v = a
        .iter()
        .zip(a[1..].iter().cloned())
        .map(|(i, j)| j - i)
        .collect::<Vec<_>>();
    v.insert(0, 1);
    v
}

fn count_ones(n: i64) -> i64 {
    G(n).iter().filter(|&e| e == &1).count() as i64
}

fn max_pn(n: i64) -> i64 {
    let v = vec![
        5, 3, 11, 23, 47, 101, 7, 13, 233, 467, 941, 1889, 3779, 7559, 15131, 53, 30323, 60647,
        121403, 242807, 19, 37, 17, 199, 29, 486041, 421, 972533, 577, 1945649, 163, 3891467, 127,
        443, 31, 7783541,
    ];
    *v[..n as usize].iter().max().unwrap()
}

fn an_over_average(n: i64) -> i64 {
    let a = A(1000001);
    let g = G(1000000);
    let v = a
        .iter()
        .cloned()
        .enumerate()
        .zip(g.iter().cloned())
        .filter(|t| t.1 > 1)
        .collect::<Vec<_>>();
    (v[..n as usize]
        .iter()
        .map(|t| (t.0).1 as f64 / (t.0).0 as f64)
        .sum::<f64>()
        / n as f64) as i64
}
