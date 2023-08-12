use regex::Regex;

fn testit(s: &str) -> usize {
    let t = Regex::new("(?i)[^word]").unwrap().replace_all(s, "");
    let mut it = "word".chars().cycle();
    let mut w = it.next().unwrap();
    let mut c = 0;
    let mut k = 0;
    for e in t.to_lowercase().chars() {
        if e == w {
            k += 1;
            if k == 4 {
                c += 1;
                k = 0;
            }
            w = it.next().unwrap();
        }
    }
    c
}
