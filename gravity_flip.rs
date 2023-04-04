fn flip(dir: char, cubes: &[u32]) -> Vec<u32> {
    let mut v = cubes.to_vec();
    v.sort();
    match dir {
        'R' => v,
        _ => v.iter().cloned().rev().collect()
    }
}