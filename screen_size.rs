fn find_screen_height(w: u64, r: &str) -> String {
    let mut v = r.split(':');
    let a = v.next().unwrap().parse::<f64>().ok().unwrap();
    let b = v.next().unwrap().parse::<f64>().ok().unwrap();
    return format!("{}x{}", w, (b / a * (w as f64)) as u64);
}
