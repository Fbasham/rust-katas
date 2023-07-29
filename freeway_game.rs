fn freeway_game(d: f64, s: f64, a: &[(f64, f64)]) -> i32 {
    let mut c = 0;
    for (t,u) in a {
        if u<=&s && t>=&0.0 || u>=&s && t<=&0.0 {continue}
        let x = d/s;
        let y = -u*t/60.0/(s-u);
        if y<x {
            c += if u<&s {1} else {-1};
        }
    }
    c
}