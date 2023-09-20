fn freed_prisoners(a: &[bool]) -> u32 {
    let mut v = a.iter().map(|e| *e).collect::<Vec<_>>();
    let mut c = 0;
    for i in 0..v.len() {
        if i==0 && !v[i] {break}
        if v[i] {
            c += 1;
            v = v.iter().map(|x| !x).collect();
        }
    }
    c
}