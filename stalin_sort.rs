fn stalin_sort(a: &mut Vec<i32>) {
    let mut i = 1;
    while i<a.len() {
        if a[i]<a[i-1] {
            a.remove(i);
            continue;
        }
        i += 1;
    }
}