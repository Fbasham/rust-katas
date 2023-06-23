use itertools::Itertools;

fn separate_liquids(a: &[Vec<char>]) -> Vec<Vec<char>> {
    if a.len() < 1 {
        return a.to_vec();
    }
    let v = a
        .iter()
        .flatten()
        .cloned()
        .sorted_by_key(|k| match k {
            'H' => 136,
            'W' => 100,
            'A' => 87,
            'O' => 80,
            _ => 0,
        })
        .collect::<Vec<_>>();
    v.chunks(a[0].len()).map(|e| e.to_vec()).collect()
}
