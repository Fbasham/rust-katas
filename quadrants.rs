fn quadrant(x: i32, y: i32) -> i32 {
    if x >= 0 && y >= 0 {
        1
    } else if x < 0 && y >= 0 {
        2
    } else if x < 0 && y < 0 {
        3
    } else {
        4
    }
}
