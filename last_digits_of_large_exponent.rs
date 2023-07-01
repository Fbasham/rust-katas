fn last_digit(s1: &str, s2: &str) -> i32 {
    if s2 == "0" {
        return 1;
    }
    let d = vec![
        [0, 0, 0, 0],
        [1, 1, 1, 1],
        [2, 4, 8, 6],
        [3, 9, 7, 1],
        [4, 6, 4, 6],
        [5, 5, 5, 5],
        [6, 6, 6, 6],
        [7, 9, 3, 1],
        [8, 4, 2, 6],
        [9, 1, 9, 1],
    ];
    let k = *&s1[s1.len().saturating_sub(1)..].parse::<usize>().unwrap();
    let x = ((*&s2[s2.len().saturating_sub(2)..].parse::<i32>().unwrap() % 4 - 1 + 4) % 4) as usize;
    d[k][x]
}
