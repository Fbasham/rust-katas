fn minimum(a: &[i32]) -> i32 {
    *a.into_iter().min().unwrap()
}

fn maximum(a: &[i32]) -> i32 {
    *a.into_iter().max().unwrap()
}