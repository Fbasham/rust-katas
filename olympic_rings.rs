fn olympic_ring(s: &str) -> String {
    let n = s
        .chars()
        .map(|e| match e {
            'q' => 1,
            'Q' => 1,
            'R' => 1,
            'o' => 1,
            'O' => 1,
            'p' => 1,
            'P' => 1,
            'a' => 1,
            'A' => 1,
            'd' => 1,
            'D' => 1,
            'g' => 1,
            'b' => 1,
            'B' => 2,
            'e' => 1,
            _ => 0,
        })
        .sum::<i32>()
        / 2;
    match n {
        0 | 1 => "Not even a medal!".to_string(),
        2 => "Bronze!".to_string(),
        3 => "Silver!".to_string(),
        _ => "Gold!".to_string(),
    }
}
