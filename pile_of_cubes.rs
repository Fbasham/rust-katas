fn find_nb(mut n: u64) -> i32 {
    let mut i = 1;
    while n>0 {
        let v = i*i*i;
        if v>n {return -1}
        n -= v;
        i += 1;
    }
    i as i32 -1
}s