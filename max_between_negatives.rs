fn max_sum_between_two_negatives(v: &Vec<i32>) -> Option<i32> {
    let mut s = 0;
    let mut f = 0;
    let mut m = 0;
    for e in v.clone() {
        if e<0 {
            f += 1;
            if f==1 {continue;}
            else {
                m = m.max(s);
                s = 0;
            }
        }
        else if f>0 {s += e;}
    }
    if f<2 {None} else {Some(m)}
}