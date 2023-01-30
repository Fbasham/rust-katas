use either::Either;

fn div_con(a: &[Either<i32, String>]) -> i32 {
    a.iter()
        .map(|e| {
            if e.is_left() {
                e.clone().unwrap_left()
            } else {
                e.clone().unwrap_right().parse::<i32>().unwrap() * -1
            }
        })
        .sum()
}
