fn wall_paper(l: f64, w: f64, h: f64) -> String {
    let v = vec![
        "zero",
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine",
        "ten",
        "eleven",
        "twelve",
        "thirteen",
        "fourteen",
        "fifteen",
        "sixteen",
        "seventeen",
        "eighteen",
        "nineteen",
        "twenty",
    ];
    match l > 0.0 && w > 0.0 && h > 0.0 {
        true => v[(1.15 * (2.0 * l * h + 2.0 * w * h) / 5.2).ceil() as usize].to_string(),
        false => "zero".to_string(),
    }
}
