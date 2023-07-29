use itertools::Itertools;

fn up_down_col_sort(a: &[Vec<i32>]) -> Vec<Vec<i32>> {
    let v = a.iter().flatten().sorted().collect::<Vec<_>>();
    let r = v
        .chunks(a.len())
        .enumerate()
        .map(|(i, t)| match i % 2 {
            0 => t.iter().cloned().sorted().collect::<Vec<_>>(),
            _ => t.iter().cloned().sorted().rev().collect::<Vec<_>>(),
        })
        .collect::<Vec<_>>();
    (0..a.len())
        .map(|i| r.iter().map(|t| *t[i]).collect::<Vec<_>>())
        .collect::<Vec<_>>()
}
