fn sum_of_minimums(a: [[u8; 4]; 4]) -> u8 {
    a.iter().map(|e| e.iter().min().unwrap()).sum()
}
