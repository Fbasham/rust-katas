pub fn sum_strings(x: &str, y: &str) -> String {
    let mut r = String::new();
    let mut a = x.bytes().rev();
    let mut b = y.bytes().rev();
    let mut c = 0;
    for _ in 0..x.len().max(y.len()) {
        let k = a.next().unwrap_or(48)+b.next().unwrap_or(48)-96+c;
        r = format!("{}{r}",k%10);
        c = k/10;
    }
    r = format!("{}{r}",if c>0 {c} else {0}).trim_start_matches("0").to_string();
    if r=="" {"0".to_string()} else {r}
}