fn sequence(n: usize) -> i64 {
    i64::from_str_radix(&format!("{n:b}"), 3).unwrap()
}
