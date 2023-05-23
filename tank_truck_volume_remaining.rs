use core::f64::consts::PI;

fn tank_vol(h: i32, d: i32, vt: i32) -> i32 {
    let r = (d as f64) / 2.0;
    let l = (vt as f64) / PI / r / r;
    let a = r * r * (1.0 - (h as f64) / r).acos()
        - (r - (h as f64)) * (r * r - (r - (h as f64)).powf(2.0)).sqrt();
    (l * a + 0.0000001) as i32
}
