fn match_counts(a: &[String], b: &[String]) -> Vec<usize> {
    b.iter()
        .map(|s| a.iter().filter(|&e| e == s).count())
        .collect()
}
