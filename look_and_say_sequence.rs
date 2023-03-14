fn get_lines(n: usize) -> String {
    if n < 1 {
        return String::new();
    }
    let mut v = vec!["1".to_string()];
    for i in 1..n {
        let mut p = ' ';
        let mut c = 0;
        let mut t = String::new();
        for e in (v[i - 1].to_owned() + " ").chars() {
            if p == ' ' || e == p {
                c += 1;
            } else {
                t = format!("{}{}{}", t, c, p);
                c = 1;
            }
            p = e;
        }
        v.push(t.trim().to_string())
    }
    v.join(",")
}
