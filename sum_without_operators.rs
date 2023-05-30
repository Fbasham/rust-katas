fn add(x: i32, y: i32) -> i32 {
    if y == 0 {
        x
    } else {
        add(x ^ y, (x & y) << 1)
    }
}
