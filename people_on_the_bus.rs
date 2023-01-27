fn number(bus_stops: &[(i32, i32)]) -> i32 {
    let mut x = 0;
    let mut y = 0;
    for (i, o) in bus_stops.iter() {
        x += i;
        y += o;
    }
    return x - y;
}
