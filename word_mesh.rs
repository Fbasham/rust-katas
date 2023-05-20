fn word_mesh(a: &[&str]) -> Option<String> {
    let mut r = String::new();
    for (x,y) in a.iter().zip(&a[1..]) {
        let mut f = 0;
        for i in (1..=x.len().min(y.len())).rev() {
            if x[x.len()-i..]==y[..i] {
                r += &y[..i];
                f = 1;
                break;
            }
        }
        if f==0 {return None}
    }
    Some(r)
}