use itertools::Itertools;
use regex::*;

fn block_pushing(a: &[char], n: u32) -> Vec<char> {
    let mut s = a.iter().cloned().join("");
    let re = Regex::new(r">+[^\-]*-").unwrap();
    for _ in 0..n {
        s = re.replace_all(&s, |c: &Captures| {
            let e = c.get(0).unwrap().as_str();
            format!("-{}",&e[..e.len()-1])
        }).to_string()
    }
    s.chars().collect()
}