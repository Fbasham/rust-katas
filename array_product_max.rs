use itertools::Itertools;

fn solve(a: &[Vec<i32>]) -> i32 {
    a.iter()
        .cloned()
        .multi_cartesian_product()
        .map(|t| t.iter().product1::<i32>().unwrap())
        .max()
        .unwrap()
}
