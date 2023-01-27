use itertools::Itertools;

fn min_value(mut v: Vec<i32>) -> i32 {
    v.iter()
        .unique()
        .sorted()
        .map(|e| e.to_string())
        .collect::<Vec<String>>()
        .join("")
        .parse()
        .unwrap()
}
