use itertools::Itertools;

fn longest_comb(a: &[i32], s: &str) -> Vec<Vec<i32>> {
    for i in (3..=a.len()).rev() {
        let mut v: Vec<Vec<i32>> = vec![];
        for c in a.iter().combinations(i) {
            if c.iter().zip(c.iter().skip(1)).all(|(i, j)| {
                if s.replace(" ", "") == "<<" {
                    j > i
                } else {
                    j < i
                }
            }) {
                v.push(c.iter().cloned().map(|e| *e).collect());
            }
        }
        if v.len() > 0 {
            return v;
        }
    }
    vec![]
}
