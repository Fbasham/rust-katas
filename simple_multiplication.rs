fn simple_multiplication(n: u8) -> u8 {
    return n * (if n % 2 == 0 { 8 } else { 9 });
}
