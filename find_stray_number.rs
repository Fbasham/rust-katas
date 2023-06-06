fn stray(a: &[u32]) -> u32 {
    *a.iter()
        .find(|e| a.iter().filter(|x| e == x).count() == 1)
        .unwrap()
}
