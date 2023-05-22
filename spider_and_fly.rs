use std::f64::consts::PI;

fn f(s: &str) -> (f64, f64) {
    let t = "CBAHGFED";
    let i = t.split("").position(|e| e == &s[..1]).unwrap() as f64;
    let r = i * PI / 4.0;
    let k = s[1..].parse::<f64>().unwrap();
    (k * r.cos(), k * r.sin())
}

fn spider_to_fly(spider: &str, fly: &str) -> f64 {
    let a = f(spider);
    let b = f(fly);
    ((a.0 - b.0).powf(2.0) + (a.1 - b.1).powf(2.0)).sqrt()
}
