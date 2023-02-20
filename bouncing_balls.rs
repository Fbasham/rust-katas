fn bouncing_ball(h: f64, b: f64, w: f64) -> i32 {
    if h <= 0.0 || b <= 0.0 || b >= 1.0 || w >= h {
        return -1;
    }
    if h < w {
        0
    } else {
        2 + bouncing_ball(h * b, b, w)
    }
}
