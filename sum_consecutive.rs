type Number = i32;
type Numbers = Vec<Number>;

fn sum_consecutives(v: &Numbers) -> Numbers {
    let mut r = vec![];
    let mut p = -1000;
    let mut s = 0;
    for e in v.iter() {
        if e == &p {
            s += e;
        } else {
            r.push(s);
            p = *e;
            s = *e;
        }
    }
    r.push(s);
    r[1..].to_vec()
}
