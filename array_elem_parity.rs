fn solve(v: &Vec<i32>) -> i32 {
    *v.iter().find(|&e| !v.contains(&-e)).unwrap()
}
