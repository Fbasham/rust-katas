use regex::*;

fn f(s: &str) -> String {
    let mut r = String::new();
    let mut c = 1;
    let mut i = (s.len() - 1) as i32;
    while i >= 0 {
        let x = s.chars().nth(i as usize).unwrap() as i32 - 48 + c;
        r = (x % 10).to_string() + &r;
        c = x / 10;
        i -= 1;
    }
    if c > 0 {
        r = c.to_string() + &r;
    }
    r
}

fn increment_string(s: &str) -> String {
    let re = Regex::new(r"(\d+$)").unwrap();
    if !re.is_match(s) {
        return s.to_string() + "1";
    }
    re.replace(s, |c: &Captures| f(c.get(1).unwrap().as_str()))
        .to_string()
}
