use std::collections::*;
use regex::Regex;

fn phone(s: &str, num: &str) -> String {
    let mut d = HashMap::new();
    for e in s.lines() {
        let p = Regex::new(r"\+\d{1,2}-\d{3}-\d{3}-\d{4}").unwrap().captures(e).unwrap().get(0).unwrap().as_str();
        if &p[1..]==num && d.contains_key(&p[1..]) {
            return format!("Error => Too many people: {}",num);
        }
        let n = Regex::new(r"<.*?>").unwrap().captures(e).unwrap().get(0).unwrap().as_str();
        let mut a = e.replace(p,"").replace(n,"").trim().to_string();
        a = Regex::new(r"[^a-zA-Z\d\-\. ]").unwrap().replace_all(&a," ").to_string();
        a = Regex::new(r"\s\s+").unwrap().replace_all(&a," ").trim().to_string();
        d.insert(&p[1..],format!("Phone => {}, Name => {}, Address => {}",&p[1..],&n[1..n.len()-1],a));
    }
    if d.contains_key(num) {return d.get(num).unwrap().to_string();}
    format!("Error => Not found: {}",num)
}