fn odd_or_even(a: Vec<i32>) -> String {
    match a.iter().sum::<i32>() % 2 {
        0 => "even".to_string(),
        _ => "odd".to_string(),
    }
}
