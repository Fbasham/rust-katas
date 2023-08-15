fn two_are_positive(a: i32, b: i32, c: i32) -> bool {
    if a>0 && b>0 && c<1 || a>0 && c>0 && b<1 || b>0 && c>0 && a<1 {
        return true;
    }
    false
}