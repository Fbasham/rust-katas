fn multiply(ia: &str, ib: &str) -> String {
    let a = ia.chars().map(|e| e.to_string().parse::<i32>().unwrap()).collect::<Vec<i32>>();
    let b = ib.chars().map(|e| e.to_string().parse::<i32>().unwrap()).collect::<Vec<i32>>();
    let mut v = vec![];
    let mut m = 0;
    for i in (0..b.len()).rev() {
        let mut t = String::new();
        let mut c = 0;
        for j in (0..a.len()).rev() {
            let x = b[i]*a[j]+c;
            t = format!("{}{}",x%10,t);
            c = x/10;
        }
        t = format!("{c}{}{}",t,"0".repeat(b.len()-i-1));
        v.push(t.to_string());
        m = m.max(t.len());
    }
    v = v.iter().map(|e| format!("{}{}","0".repeat(m-e.len()),e)).collect();
    let mut c = 0;
    let mut r = String::new();
    for i in (0..m).rev() {
        let x: i32 = (0..v.len()).map(|j| v[j].chars().nth(i).unwrap().to_digit(10).unwrap() as i32).sum::<i32>()+c;
        r = format!("{}{}",x%10,r);
        c = x/10;
    }
    r = format!("{}",r).trim_start_matches("0").to_string();
    if r=="" {"0".to_string()} else {r}
}