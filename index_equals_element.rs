fn index_equals_value(a: &[i32]) -> i32 {
    let mut lo = 0 as i32;
    let mut hi = a.len() as i32 - 1;
    if hi < 0 || a[0] > 0 || a[hi as usize] < hi {
        return -1;
    }
    while lo + 1 < hi {
        let m = (lo + hi) >> 1;
        if a[m as usize] >= m {
            hi = m
        } else {
            lo = m
        }
    }
    if a[lo as usize] == lo {
        lo
    } else if a[hi as usize] == hi {
        hi
    } else {
        -1
    }
}
