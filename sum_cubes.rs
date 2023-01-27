fn sum_cubes(n: u32) -> u32 {
    (1..=n).fold(0, |a, c| a + c.pow(3))
}
