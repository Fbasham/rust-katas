use std::collections::HashMap;
use regex::Regex;

fn encode_resistor_colors(s: &str) -> String {
    let d = HashMap::from([
        ("0","black"),("1","brown"),("2","red"),("3","orange"),("4","yellow"),
        ("5","green"),("6","blue"),("7","violet"),("8","gray"),("9","white")
    ]);
    let re = Regex::new(r"[^\d\.]").unwrap();
    let t = (re.replace_all(s,"").parse::<f64>().unwrap()*(if s.contains('M') {1000000.0} else if s.contains('k') {1000.0} else {1.0})).round().to_string();
    let n = &t[2..t.len()].len();
    format!("{} {} {} gold",d.get(&t[..1]).unwrap(),d.get(&t[1..2]).unwrap(),d.get(&n.to_string().as_str()).unwrap())
}