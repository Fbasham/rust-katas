fn remove_every_other(a: &[u8]) -> Vec<u8> {
    a.iter()
        .enumerate()
        .filter(|(i, e)| i % 2 == 0)
        .map(|(i, e)| e)
        .cloned()
        .collect()
}
