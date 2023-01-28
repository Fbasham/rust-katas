fn even_numbers(a: &Vec<i32>, n: usize) -> Vec<i32> {
    a.iter()
        .filter(|&e| e % 2 == 0)
        .copied()
        .rev()
        .take(n)
        .collect::<Vec<i32>>()
        .iter()
        .copied()
        .rev()
        .collect::<Vec<i32>>()
}
