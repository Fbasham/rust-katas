fn min_max(a: &[i32]) -> (i32, i32) {
    (*a.iter().min().unwrap(), *a.iter().max().unwrap())
}
