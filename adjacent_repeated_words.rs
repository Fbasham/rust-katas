fn count_adjacent_pairs(s: &str) -> usize {
    let mut c = 0;
    let mut k = 0;
    let mut p = "".to_string();
    for e in s.split(" "){
        let t = e.to_lowercase();
        if p=="" || t==p {
            k += 1;
        }
        else {
            if k>1 {
                c += 1;
            }
            k = 1;
        }
        p = t;
    }
    c+(if k>1 {1} else {0})
}   