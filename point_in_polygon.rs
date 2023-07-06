type Point = (f32, f32);

fn point_in_poly(v: &[Point], c: Point) -> bool {
    let mut r = false;
    let mut j = v.len() - 1;
    for i in 0..v.len() {
        let xi = v[i].0;
        let yi = v[i].1;
        let xj = v[j].0;
        let yj = v[j].1;
        if (((yi > c.1) != (yj > c.1)) && c.0 < ((xj - xi) * (c.1 - yi)) / (yj - yi) + xi) {
            r = !r;
        }
        j = i;
    }
    r
}
