use regex::Regex;

fn decode(s: &str) -> String {
    let n = Regex::new(r"\D")
        .unwrap()
        .replace_all(s, "")
        .parse::<i32>()
        .unwrap();
    let t = Regex::new(r"\d").unwrap().replace_all(s, "").to_string();
    let mut r = String::new();
    for c in t.bytes() {
        let mut x = 0;
        for i in 0..26 {
            if i * n % 26 == c as i32 - 97 {
                if x > 0 {
                    return "Impossible to decode".to_string();
                }
                x = i as u8 + 97;
            }
        }
        r.push(x as char);
    }
    r
}
