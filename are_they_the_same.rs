use itertools::Itertools;

fn comp(a: Vec<i64>, b: Vec<i64>) -> bool {
    a.len() == b.len()
        && a.iter()
            .map(|e| e.pow(2))
            .sorted()
            .zip(b.iter().sorted())
            .all(|e| e.0 == *e.1)
}
