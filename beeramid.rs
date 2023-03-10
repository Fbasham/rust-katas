fn beeramid(bonus: i32, price: f32) -> usize {
    if bonus < 1 {
        return 0;
    }
    let mut b = bonus as f32;
    let mut n = 0.0;
    while b > 0.0 {
        let p = n * n * price;
        if p > b {
            break;
        }
        n += 1.0;
        b -= p;
    }
    n as usize - 1
}
