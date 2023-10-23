fn game(n: u32, m: u32) -> &'static str {
    if m % 2 == 0 && n % 2 == 0 {
        return "second";
    }
    if (n % 2 == 0 && m % 2 == 1) || m == 2 {
        "second"
    } else {
        "first"
    }
}
