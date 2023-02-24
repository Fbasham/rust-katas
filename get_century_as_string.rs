fn what_century(year: &str) -> String {
    let mut n = year.parse::<i32>().unwrap();
    n = if n % 1000 == 0 { n / 100 } else { n / 100 + 1 };
    n.to_string()
        + match n % 10 {
            1 if n != 11 => "st",
            2 if n != 12 => "nd",
            3 if n != 13 => "rd",
            _ => "th",
        }
}
