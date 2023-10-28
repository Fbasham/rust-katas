fn generate_pairs(m: i16, n: i16) -> Vec<(i16, i16)> {
    (m..=n).map(|i| (m..=n).filter(move |j| i<=*j).map(move |j| (i,j))).flatten().collect()
}