use itertools::Itertools;

pub fn modes(a: &[i32]) -> Vec<i32> {
    let d = a.iter().cloned().counts();
    let m = d.values().max().unwrap_or(&0);
    d.keys()
        .cloned()
        .filter(|&e| d.get(&e).unwrap() == m)
        .collect()
}
