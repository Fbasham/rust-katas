use regex::Regex;

fn tire_rotations(s: &str, d: i32) -> f64 {
    let a: Vec<f64> = Regex::new("\\d+").unwrap().find_iter(s).map(|m| m.as_str().parse::<f64>().unwrap()).collect();
    d as f64*1000000.0/(a[2]*25.4+2.0*(a[0]*a[1]/100.0))/std::f64::consts::PI
}