use itertools::Itertools;

fn two_oldest_ages(a: &[u8]) -> [u8; 2] {
    let v = a.iter().map(|e| *e).sorted().rev().collect::<Vec<u8>>();
    [v[1], v[0]]
}
