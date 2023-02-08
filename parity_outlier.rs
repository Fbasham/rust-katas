fn find_outlier(a: &[i32]) -> i32 {
    let e = a.iter().filter(|&x| (x % 2).abs() == 0).collect::<Vec<_>>();
    let o = a.iter().filter(|&x| (x % 2).abs() == 1).collect::<Vec<_>>();
    if e.len() == 1 {
        *e[0]
    } else {
        *o[0]
    }
}
