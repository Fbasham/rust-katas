use itertools::Itertools;

fn get_pins(s: &str) -> Vec<String> {
    let v = vec![
        vec![0, 8],
        vec![1, 2, 4],
        vec![2, 1, 3, 5],
        vec![3, 2, 6],
        vec![4, 1, 5, 7],
        vec![5, 2, 4, 6, 8],
        vec![6, 3, 5, 9],
        vec![7, 4, 8],
        vec![8, 5, 7, 9, 0],
        vec![9, 6, 8],
    ];
    s.chars()
        .map(|i| v[i as usize - 48].iter())
        .multi_cartesian_product()
        .map(|x| x.iter().map(|e| e.to_string()).join(""))
        .sorted()
        .collect::<Vec<_>>()
}
