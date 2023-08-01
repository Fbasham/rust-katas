use itertools::Itertools;

fn pf(mut n: i64) -> Vec<i64> {
    n = n.abs();
    let mut v = vec![];
    let mut i = 2;
    while i * i <= n {
        while n % i == 0 {
            if !v.contains(&i) {
                v.push(i);
            }
            n /= i;
        }
        i += 1;
    }
    if n > 1 {
        v.push(n)
    }
    v
}

fn sum_of_divided(l: Vec<i64>) -> Vec<(i64, i64)> {
    let u = l
        .iter()
        .cloned()
        .map(|e| pf(e))
        .flatten()
        .unique()
        .sorted()
        .collect::<Vec<_>>();
    u.iter()
        .cloned()
        .map(|e| (e, l.iter().cloned().filter(|&x| (x % e).abs() == 0).sum()))
        .collect()
}
