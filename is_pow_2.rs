fn power_of_two(x: u64) -> bool {
    (x as f64).log2()%1.0==0.0
}