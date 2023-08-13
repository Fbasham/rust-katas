fn wave_sort(a: &mut [i32]) {
    if a.len() == 0 {
        return;
    }
    a.sort();
    for i in (0..a.len() - 1).step_by(2) {
        (a[i], a[i + 1]) = (a[i + 1], a[i]);
    }
}
