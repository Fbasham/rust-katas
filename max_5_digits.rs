fn largest_five_digit_number(num: &str) -> u32 {
    let s = String::from(num);
    let mut m = 0;
    for i in 0..s.len() - 4 {
        let n = s[i..i + 5].parse::<u32>().unwrap();
        if n > m {
            m = n;
        }
    }
    m
}
