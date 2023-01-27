fn is_triangle(a: i64, b: i64, c: i64) -> bool {
    a + b > c && a + c > b && b + c > a
}
