fn days_represented(trips: &[(u32, u32)]) -> u32 {
    (0..366)
        .filter(|i| trips.iter().any(|(x, y)| i >= x && i <= y))
        .count() as u32
}
