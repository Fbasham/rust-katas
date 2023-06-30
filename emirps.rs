fn f(n: u32) -> bool {
    if n % 2 == 0 {
        return n == 2;
    }
    for i in (3..=(n as f64).sqrt() as u32).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn find_emirp(n: u32) -> (u32, u32, u64) {
    let v = (0..n)
        .filter(|&e| {
            let s = e.to_string().chars().rev().collect::<String>();
            f(e) && f(s.parse().unwrap()) && s != e.to_string()
        })
        .collect::<Vec<_>>();
    (
        v.len() as u32,
        v.iter().cloned().max().unwrap_or(0),
        v.iter().map(|e| *e as u64).sum::<u64>(),
    )
}
