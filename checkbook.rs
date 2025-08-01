use regex::Regex;

fn balance(s: &str) -> String {
    let re = Regex::new(r"[^a-zA-Z\d. \n]").unwrap();
    let t = re.replace_all(s, "");
    let a = t.split("\n").filter(|&e| e != "").collect::<Vec<_>>();
    let mut r = vec![];
    let mut n = a[0].parse::<f64>().unwrap();
    let mut v = 0.0;
    let mut c = 0.0;
    for i in 0..a.len() {
        match i {
            0 => r.push(format!("Original Balance: {:.2}", n)),
            _ => {
                let t = a[i].split(" ").collect::<Vec<_>>();
                let x = t[2].parse::<f64>().unwrap();
                let k = n - x;
                n -= x;
                v += x;
                c += 1.0;
                r.push(format!("{} {} {:.2} Balance {:.2}", t[0], t[1], x, k));
            }
        }
    }
    r.push(format!("Total expense  {:.2}", v));
    r.push(format!("Average expense  {:.2}", v / c));
    r.join("\n")
}
