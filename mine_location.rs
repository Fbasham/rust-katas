pub fn mine_location(a: &[Vec<u8>]) -> (usize, usize) {
    (0..a.len())
        .flat_map(|i| (0..a[i].len()).map(move |j| (i, j)))
        .find(|(i, j)| a[*i][*j] == 1)
        .unwrap()
}
