fn collatz(mut n: u32) -> String {
    let mut v = vec![n.to_string()];
    while n != 1 {
        n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
        v.push(n.to_string());
    }
    v.join("->")
}
