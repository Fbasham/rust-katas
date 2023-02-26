fn good_vs_evil(good: &str, evil: &str) -> String {
    let g = vec![1, 2, 3, 3, 4, 10];
    let e = vec![1, 2, 2, 2, 3, 5, 10];
    let a = good
        .split(" ")
        .enumerate()
        .map(|(i, x)| x.parse::<i32>().unwrap() * g[i])
        .sum::<i32>();
    let b = evil
        .split(" ")
        .enumerate()
        .map(|(i, x)| x.parse::<i32>().unwrap() * e[i])
        .sum::<i32>();
    (if a == b {
        "Battle Result: No victor on this battle field"
    } else if a > b {
        "Battle Result: Good triumphs over Evil"
    } else {
        "Battle Result: Evil eradicates all trace of Good"
    })
    .to_string()
}
