fn find_difference(a: &[i32; 3], b: &[i32; 3]) -> i32 {
    let pa: i32 = a.iter().product();
    let pb: i32 = b.iter().product();
    (pa - pb).abs()
}
