fn solution(a: &[Vec<char>]) -> bool {
    let y = (0..a.len()).position(|i| a[i].contains(&'x')).unwrap();
    let x = a[y].iter().position(|e| e == &'x').unwrap();
    (0..y).any(|i| a[i][x] == 'v')
        || (y..a.len()).any(|i| a[i][x] == '^')
        || (0..x).any(|i| a[y][i] == '>')
        || (x..a[0].len()).any(|i| a[y][i] == '<')
}
