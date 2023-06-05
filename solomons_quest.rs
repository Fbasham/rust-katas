mod quest {
    pub fn solomons_quest(v: Vec<(i32, i32, i32)>) -> (i32, i32) {
        let mut x = 0;
        let mut y = 0;
        let mut k = 0;
        for (t, m, d) in v {
            k += t;
            if m == 0 || m == 2 {
                y += (if m == 2 { -1 } else { 1 }) * d * 2_i32.pow(k as u32);
            } else {
                x += (if m == 3 { -1 } else { 1 }) * d * 2_i32.pow(k as u32);
            }
        }
        (x, y)
    }
}
