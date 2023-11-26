use itertools::Itertools;

fn sum_or_product(a: &[i64], n: usize) -> String {
    let x = a.iter().sorted().rev().take(n).sum::<i64>();
    let y = a.iter().sorted().take(n).product();
    if x == y {
        "same".to_string()
    } else if x > y {
        "sum".to_string()
    } else {
        "product".to_string()
    }
}
