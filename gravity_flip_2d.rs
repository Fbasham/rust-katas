use itertools::Itertools;

fn flip(k: char, a: Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let u = (0..a[0].len())
        .map(|i| a.iter().map(|t| t[i]).sorted().rev().collect::<Vec<u32>>())
        .collect::<Vec<Vec<u32>>>();
    let d = (0..a[0].len())
        .map(|i| a.iter().map(|t| t[i]).sorted().collect::<Vec<u32>>())
        .collect::<Vec<Vec<u32>>>();
    match k {
        'R' => a
            .iter()
            .cloned()
            .map(|e| e.iter().cloned().sorted().collect())
            .collect(),
        'L' => a
            .iter()
            .cloned()
            .map(|e| e.iter().cloned().sorted().rev().collect())
            .collect(),
        'U' => (0..a.len())
            .map(|i| u.iter().map(|t| t[i]).collect())
            .collect(),
        _ => (0..a.len())
            .map(|i| u.iter().map(|t| t[i]).collect())
            .rev()
            .collect(),
    }
}
