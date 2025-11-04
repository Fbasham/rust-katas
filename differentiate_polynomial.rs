use regex::Regex;

fn differentiate(s: &str, k: i64) -> i64 {
    let mut r = 0;
    for c in Regex::new(r"(-?\d*)x\^?(\d*)").unwrap().captures_iter(s) {
        let sx = &c[1];
        let sy = &c[2];
        let x = if sx=="" {1} else if sx=="-" {-1} else {sx.parse::<i64>().unwrap()};
        let y = if sy=="" {1} else if sy=="-" {-1} else {sy.parse::<i64>().unwrap()};
        r += x*y*k.pow(y as u32-1);
    }
    r
}