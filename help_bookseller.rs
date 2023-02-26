fn stock_list(a: Vec<&str>, c: Vec<&str>) -> String {
    let mut v = vec![];
    for k in &c {
        let mut n = 0;
        for e in &a {
            if e.starts_with(k) {
                n += e.split(" ").last().unwrap().parse::<i32>().unwrap();
            }
        }
        v.push(format!("({} : {})",k,n));
    }
    match a.len()==0 || c.len()==0 {
        true => String::new(),
        _ => v.join(" - ")
    }
}