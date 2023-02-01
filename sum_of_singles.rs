fn repeats(v: &Vec<i32>) -> i32 {
    v.iter()
        .filter(|&e| v.iter().filter(|&x| x == e).count() == 1)
        .sum()
}
