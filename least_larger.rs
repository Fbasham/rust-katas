use itertools::Itertools;

fn least_larger(a: &[i32], i: usize) -> Option<usize> {
    let x = a.iter().sorted().filter(|&e| e > &a[i]).next();
    match x {
        Some(x) => a.iter().position(|e| e == x),
        None => None,
    }
}
