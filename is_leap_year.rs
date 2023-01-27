fn is_leap_year(y: i32) -> bool {
    y % 400 == 0 || y % 4 == 0 && y % 100 != 0
}
