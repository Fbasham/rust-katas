fn move_zeros(a: &[u8]) -> Vec<u8> {
    a.iter()
        .filter(|&e| *e != 0)
        .chain(a.iter().filter(|&e| *e == 0))
        .map(|e| *e)
        .collect()
}
