fn goalkeeper_reflex(g: i32, b: i32) -> i32 {
    if g == b {
        return g;
    }
    if b < g {
        return (g - 1).max(0);
    }
    return (g + 1).min(10);
}
