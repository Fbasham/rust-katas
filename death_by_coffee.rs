fn coffee_limits(y: u32, m: u32, d: u32) -> (u32, u32) {
    let mut nc = format!("{y:04}{m:02}{d:02}").parse::<i128>().unwrap();
    let mut nd = nc;
    let mut c: u32 = 0;
    let mut d: u32 = 0;
    let mut fc = false;
    let mut fd = false;
    let cafe = i128::from_str_radix("cafe",16).unwrap();
    let decaf = i128::from_str_radix("decaf",16).unwrap();
    for _ in 0..5000 {
        if fc==false {
            nc += cafe;
            c += 1;
            if format!("{nc:x}").contains("dead") {
                fc = true;
            }
        }
        if fd==false {
            nd += decaf;
            d += 1;
            if format!("{nd:x}").contains("dead") {
                fd = true
            }
        }
    }
    (if c==5000 {0} else {c},if d==5000 {0} else {d})
}