fn gould() -> impl Iterator<Item = u8> {
    (0..).map(|n: u32| n.count_ones() as u8)
}
