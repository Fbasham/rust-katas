use itertools::Itertools;

fn sjf(a: &[usize], k: usize) -> usize {
    a.into_iter()
        .enumerate()
        .sorted_by_key(|t| t.1)
        .take_while(|t| t.0 != k)
        .map(|t| t.1)
        .sum::<usize>()
        + &a[k]
}
