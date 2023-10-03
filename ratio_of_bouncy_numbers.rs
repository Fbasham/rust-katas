use lazy_static::lazy_static;

lazy_static! {
    static ref A: Vec<u32> = { (100..2000000).filter(|&e| f(e)).collect::<Vec<_>>() };
}

fn f(n: u32) -> bool {
    let s = n.to_string();
    !s.chars().zip(s.chars().skip(1)).all(|(i, j)| i <= j)
        && !s.chars().zip(s.chars().skip(1)).all(|(i, j)| i >= j)
}

fn bouncy_ratio(r: f64) -> Option<u32> {
    if r == 0.0 {
        return Some(1);
    }
    if r > 0.99 {
        return None;
    }
    Some(
        *A.iter()
            .enumerate()
            .find(|(i, e)| (*i as f64 + 1.0) / (**e as f64) >= r)
            .unwrap()
            .1,
    )
}
