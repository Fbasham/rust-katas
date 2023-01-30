fn pairs(v: &Vec<i32>) -> usize {
    let mut c = 0;
    for i in 0..v.len() / 2 {
        if (v[2 * i] - v[2 * i + 1]).abs() == 1 {
            c += 1;
        }
    }
    c
}
