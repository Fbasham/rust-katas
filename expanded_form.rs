fn expanded_form(n: u64) -> String {
    let mut v = vec![];
    let mut t = n;
    let mut i = 0;
    while t > 0 {
        let x = t % 10;
        t /= 10;
        if x > 0 {
            v.insert(0, (10_u64.pow(i) * x).to_string())
        }
        i += 1;
    }
    v.join(" + ")
}
