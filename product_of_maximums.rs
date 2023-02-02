use itertools::Itertools;

fn max_product(mut v: Vec<i32>, k: i32) -> i32 {
    v.iter().sorted().rev().take(k as usize).product()
}
