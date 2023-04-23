use itertools::Itertools;

fn flatten_and_sort(a: &[Vec<i32>]) -> Vec<i32> {
    a.into_iter().flatten().cloned().sorted().collect()
}
