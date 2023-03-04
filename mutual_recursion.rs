fn f(n: i32) -> i32 {
    match n {
        0 => 1,
        _ => n - m(f(n - 1)),
    }
}

fn m(n: i32) -> i32 {
    match n {
        0 => 0,
        _ => n - f(m(n - 1)),
    }
}
