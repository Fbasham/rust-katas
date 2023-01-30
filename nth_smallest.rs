use itertools::Itertools;

fn nth_smallest(a: &[i32], n: usize) -> i32 {
    *a.iter().sorted().nth(n - 1).unwrap()
}
