use itertools::Itertools;

fn power(a: &[u32]) -> Vec<Vec<u32>> {
    let mut v = vec![];
    for i in 0..=a.len() {
        for c in a.iter().map(|e| *e).combinations(i) {
            v.push(c);
        }
    }
    v
}
