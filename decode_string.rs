use regex::Regex;

fn decode(s: &str) -> Result<Vec<usize>, ()> {
    if Regex::new("\\D").unwrap().is_match(s) {
        return Err(());
    }
    let mut v = vec![];
    let mut i = 0;
    while i < s.len() {
        let k = s.chars().nth(i).unwrap() as usize - 48;
        if k == 0 {
            i += 1;
            continue;
        }
        if i + k >= s.len() {
            return Err(());
        }
        let t = &s[i + 1..i + k + 1];
        v.push(t.parse::<usize>().unwrap());
        i += 1 + k;
    }
    Ok(v)
}
