fn is_odd_heavy(a: &[i32]) -> bool {
    if a.iter().filter(|&e| (e % 2).abs() == 1).count() == 0 {
        return false;
    }
    a.iter()
        .filter(|&e| (e % 2).abs() == 1)
        .all(|e| a.iter().all(|x| (x % 2).abs() == 1 || e > x))
}
