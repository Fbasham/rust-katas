use std::iter::zip;

fn adjacent_elements_product(a: &[i32]) -> i32 {
    zip(a.iter(), a.iter().skip(1))
        .map(|(i, j)| i * j)
        .max()
        .unwrap()
}
