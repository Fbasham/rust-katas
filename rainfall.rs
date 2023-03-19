fn f(s: &str) -> Vec<f64> {
    s.split_whitespace().map(|e| e.split(",").next().unwrap().parse::<f64>().unwrap_or(0.0)).filter(|&e| e>0.0).collect()
}

fn mean(town: &str, strng: &str) -> f64 {
    if !strng.split("\n").map(|e| e.split(":").next().unwrap()).any(|e| e==town) {return -1.0;}
    f(strng.split("\n").filter(|&s| s.contains(town)).next().unwrap_or("")).iter().sum::<f64>()/12.0
}
fn variance(town: &str, strng: &str) -> f64 {
    if !strng.split("\n").map(|e| e.split(":").next().unwrap()).any(|e| e==town) {return -1.0;}
    let u = mean(town,strng);
    f(strng.split("\n").filter(|&s| s.contains(town)).next().unwrap_or("")).iter().map(|e| (e-u).powf(2.0)).sum::<f64>()/12.0
}