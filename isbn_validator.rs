use regex::Regex;

pub fn validate_isbn(isbn: &str) -> bool {
    let s = Regex::new(r"\D").unwrap().replace_all(isbn, "");
    match s.len() {
        10 => {
            s[0..9]
                .chars()
                .enumerate()
                .map(|(i, e)| (i + 1) as u32 * e.to_digit(10).unwrap())
                .sum::<u32>()
                % 11
                == s[9..s.len()].parse().unwrap()
        }
        13 => {
            10 - s[0..12]
                .chars()
                .enumerate()
                .map(|(i, e)| (if i % 2 == 0 { 1 } else { 3 }) * e.to_digit(10).unwrap())
                .sum::<u32>()
                % 10
                == s[12..s.len()].parse().unwrap()
        }
        _ => false,
    }
}
