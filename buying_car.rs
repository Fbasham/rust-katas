fn nb_months(old: i32, new: i32, saving: i32, perc: f64) -> (i32, i32) {
    let mut s = 0 as f64;
    let mut i = 1;
    let mut o = old as f64;
    let mut n = new as f64;
    let mut p = perc;
    while s + o < n {
        if i > 0 && i % 2 == 0 {
            p += 0.5;
        }
        s += saving as f64;
        o *= 1.0 - p / 100.0;
        n *= 1.0 - p / 100.0;
        i += 1;
    }
    println!("{} {} {}", s, o, n);
    (i - 1, (s + o - n).round() as i32)
}
