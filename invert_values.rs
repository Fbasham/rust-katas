fn invert(a: &[i32]) -> Vec<i32> {
    return a.iter().map(|e| -e).collect();
}
