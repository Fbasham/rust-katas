fn prev_mult_of_three(n: i32) -> i32 {
    if n == 0 {
        -1
    } else if n % 3 == 0 {
        n
    } else {
        prev_mult_of_three(n / 10)
    }
}
