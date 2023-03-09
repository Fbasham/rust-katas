use regex::Regex;

fn valid_isbn10(s: &str) -> bool {
    if !Regex::new(r"^\d{10}$|^\d{9}X$").unwrap().is_match(s) {
        return false;
    }
    s.chars()
        .enumerate()
        .map(|(i, e)| (i + 1) * (if e == 'X' { 10 } else { e as usize - 48 }))
        .sum::<usize>()
        % 11
        == 0
}
