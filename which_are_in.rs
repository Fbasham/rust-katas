use itertools::Itertools;

fn in_array(a: &[&str], b: &[&str]) -> Vec<String> {
    a.iter()
        .filter(|&e| b.iter().any(|&x| x.contains(e)))
        .map(|e| e.to_owned().to_string())
        .sorted()
        .unique()
        .collect()
}
