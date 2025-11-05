use regex::Regex;

fn f(a: &str, b: &str, c: &str, d: &str, k: u8) -> bool {
    let x = a.replace("?",&k.to_string()).parse::<i32>().unwrap();
    let y = c.replace("?",&k.to_string()).parse::<i32>().unwrap();
    let z = d.replace("?",&k.to_string()).parse::<i32>().unwrap();
    for t in [a,c,d] {
        if k==0 && Regex::new(r"^-?\?.").unwrap().is_match(t) {return false}
    }
    match b {
        "+" => x+y==z,
        "-" => x-y==z,
        _ => x*y==z,
    }
}

fn solve_runes(s: &str) -> Option<u8> {
    let m = Regex::new(r"(-?[\d?]+)([\+\-\*])(-?[\d?]+)=(-?[\d?]+)").unwrap().captures(s).unwrap();
    let a = m.get(1).unwrap().as_str();
    let b = m.get(2).unwrap().as_str();
    let c = m.get(3).unwrap().as_str();
    let d = m.get(4).unwrap().as_str();
    for k in 0..10 {
        if !s.contains(&k.to_string()) && f(a,b,c,d,k) {return Some(k)}
    }
    None
}