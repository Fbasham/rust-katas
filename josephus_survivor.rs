fn g(n: i32, k: i32) -> i32 {
    if n == 1 {
        0
    } else {
        (g(n - 1, k) + k) % n
    }
}

fn josephus_survivor(n: i32, k: i32) -> i32 {
    g(n, k) + 1
}
