use itertools::Itertools;
use std::collections::HashSet;

fn part(n: i64) -> String {
    let mut q = vec![(n, n, 1)];
    let mut s = HashSet::new();
    let mut v = HashSet::new();
    while q.len() > 0 {
        let (x, y, t) = q.pop().unwrap();
        if x <= 0 || y < 0 {
            continue;
        }
        if y == 0 {
            s.insert(t);
        }
        q.push((x, y - x, t * x));
        for i in 1..y {
            let tt = format!("{} {} {}", x - i, y - i, t * i);
            if !v.contains(&tt) {
                q.push((x - i, y - i, t * i));
                v.insert(tt);
            }
        }
    }
    let a = s.iter().cloned().sorted().collect::<Vec<_>>();
    format!(
        "Range: {} Average: {:.2} Median: {:.2}",
        a[a.len() - 1] - a[0],
        (a.iter().cloned().sum::<i64>() as f64 / a.len() as f64),
        if a.len() % 2 == 1 {
            a[a.len() / 2] as f64
        } else {
            (a[a.len() / 2] + a[a.len() / 2 - 1]) as f64 / 2.0
        }
    )
}
