use regex::Regex;

fn expand(s: &str) -> String {
    let f = { |n| (1..=n).fold(1, |a, c| a * c) };
    let C = { |a, b| f(a) / f(b) / f(a - b) };
    let re = Regex::new(r"\((-?\d*)([a-z])([+\-]\d+)\)\^(\d+)").unwrap();
    let caps = re.captures(s).unwrap();
    let sa = caps.get(1).unwrap().as_str();
    let v = caps.get(2).unwrap().as_str();
    let sb = caps.get(3).unwrap().as_str();
    let sc = caps.get(4).unwrap().as_str();
    let a = match sa {
        "" => 1,
        "-" => -1,
        _ => sa.parse::<i32>().unwrap(),
    };
    let b = match sb {
        "" => 1,
        "-" => -1,
        _ => sb.parse::<i32>().unwrap(),
    };
    let c = match sc {
        "" => 1,
        "-" => -1,
        _ => sc.parse::<i32>().unwrap(),
    };
    let mut r = vec![];
    for k in 0..=c {
        let x = C(c, k) * a.pow((c - k) as u32) * b.pow(k as u32);
        let e = c - k;
        r.push(match (x, e) {
            (0, _) => String::new(),
            (_, 0) => x.to_string(),
            (1, 1) => format!("{}", v),
            (-1, 1) => format!("-{}", v),
            (_, 1) => format!("{}{}", x, v),
            (1, _) => format!("{}^{}", v, e),
            (-1, _) => format!("-{}^{}", v, e),
            _ => format!("{}{}^{}", x, v, e),
        });
    }
    r.join("+").replace("+-", "-")
}
