use std::f64;

fn solve(na: i32, nb: i32, nc: i32, da: i32, db: i32, dc: i32) -> Vec<i32> {
    const PI: f64 = 3.1415926535897932;
    let a = na as f64;
    let b = nb as f64;
    let c = nc as f64;
    let A = da;
    let B = db;
    let C = dc;
    let mut x = ((A as f64) * PI / 180.0).cos() * a;
    let mut y = ((A as f64) * PI / 180.0).sin() * a;
    x += ((B as f64 + 90.0) * PI / 180.0).cos() * b;
    y += ((B as f64 + 90.0) * PI / 180.0).sin() * b;
    x += ((C as f64 + 180.0) * PI / 180.0).cos() * c;
    y += ((C as f64 + 180.0) * PI / 180.0).sin() * c;
    let n = (x * x + y * y).sqrt().round() as i32;
    let t = y.atan2(x) * 180.0 / PI;
    let d = t as i32;
    let m = ((t % 1.0) * 60.0) as i32;
    let s = ((((t % 1.0) * 60.0) % 1.0) * 60.0) as i32;
    vec![n, d, m, s]
}
