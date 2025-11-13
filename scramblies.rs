use itertools::Itertools;

fn scramble(a: &str, b: &str) -> bool {
    let da = a.chars().counts();
    let db = b.chars().counts();
    db.iter().all(|(k,v)| da.get(k).unwrap_or(&0)>=v)
}