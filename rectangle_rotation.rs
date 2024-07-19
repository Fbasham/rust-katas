fn rectangle_rotation(a: i32, b: i32) -> i32 {
    let x = ((a * a / 2) as f64).sqrt() as i32;
    let y = ((b * b / 2) as f64).sqrt() as i32;
    (x * y + (x + y) / 2) * 2 + 1
}
