fn needs_new_glasses(v: &Vec<Vec<i32>>, r: i32) -> bool {
    v.len() as i32 * v.iter().map(|e| e.len() as i32).max().unwrap() >= r
}
