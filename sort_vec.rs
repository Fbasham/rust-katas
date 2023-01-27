use itertools::Itertools;

fn sort_numbers(a: &Vec<i32>) -> Vec<i32> {
    a.iter().map(|e| *e).sorted().collect()
}
