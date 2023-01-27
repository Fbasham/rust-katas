fn get_sum(a: i64, b: i64) -> i64 {
    let mut r = 0;
    let mut i = a;
    let mut j = b;
    if i > j {
        (i, j) = (j, i);
    }
    for x in i..j + 1 {
        r += x;
    }
    return r;
}
