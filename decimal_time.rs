fn string_to_industrial(s: &str) -> f64 {
    s.split(":").nth(0).unwrap().parse::<f64>().unwrap()
        + (s.split(":").nth(1).unwrap().parse::<f64>().unwrap() / 60.0 * 100.0).round() / 100.0
}

fn to_industrial(time: u32) -> f64 {
    (time as f64 / 60.0 * 100.0).round() / 100.0
}

fn to_normal(t: f64) -> String {
    format!("{}:{:02}", t.floor(), ((t % 1.0) * 60.0).round())
}
