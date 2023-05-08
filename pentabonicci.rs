use itertools::Itertools;
use num::bigint::BigInt;

fn count_odd_pentafib(n: u16) -> u16 {
    if n == 0 {
        return 0;
    }
    let mut a = vec![0, 1, 1, 2, 4]
        .iter()
        .map(|e| BigInt::from(*e))
        .collect::<Vec<_>>();
    for _ in 0..n.saturating_sub(4) {
        a.push(a[a.len() - 5..a.len()].iter().sum());
    }
    a.iter()
        .filter(|&e| e % 2 == BigInt::from(1))
        .unique()
        .count() as u16
}
