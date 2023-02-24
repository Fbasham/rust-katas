fn solve(v: &Vec<String>) -> i32 {
    return v
        .iter()
        .filter(|&e| match &e.parse::<i32>() {
            Ok(e) => e % 2 == 0,
            _ => false,
        })
        .count() as i32
        - v.iter()
            .filter(|&e| match &e.parse::<i32>() {
                Ok(e) => e % 2 == 1,
                _ => false,
            })
            .count() as i32;
}
