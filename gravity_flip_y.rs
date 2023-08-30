use itertools::Itertools;

fn switch_gravity(a: &[Vec<char>]) -> Vec<Vec<char>> {
    let v = (0..a[0].len())
        .map(|i| {
            let s = a.iter().map(move |t| t[i]).join("").replace("-", "");
            ("-".repeat(a.len() - s.len()) + &s)
                .chars()
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    (0..a.len())
        .map(|i| v.iter().map(|t| t[i]).collect())
        .collect()
}
