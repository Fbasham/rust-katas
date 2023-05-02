fn add_arrays(a: &[i64], b: &[i64]) -> Vec<i64> {
    if a.is_empty() && b.is_empty() {return vec![];}
    let x = a.iter().map(|e| e.to_string()).collect::<String>().parse::<i64>().unwrap_or(0);
    let y = b.iter().map(|e| e.to_string()).collect::<String>().parse::<i64>().unwrap_or(0);
    let n = x+y;
    let mut v: Vec<i64> = n.abs().to_string().chars().map(|e| e.to_digit(10).unwrap() as i64).collect();
    if n<0 {v[0] *= -1;}
    v
}