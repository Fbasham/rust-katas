use regex::Regex;

fn balance(s: &str) -> String {
    let re = Regex::new(r"[^\d\w\. ]").unwrap();
    let a = s.split("\n").map(|e| e.to_string()).collect::<Vec<_>>();
    let mut b = 0.0;
    let mut x = 0.0;
    let mut v = vec![];
    for e in a {
        let t = re.replace_all(&e,"").into_owned().trim().to_string();
        if t.is_empty() {continue}
        if v.len()==0 {
            b = t.parse::<f64>().unwrap();
            v.push(format!("Original Balance: {:.2}",b));
        }
        else {
            let c = t.split(" ").last().unwrap().parse::<f64>().unwrap();
            let d = t.split(" ").collect::<Vec<_>>();
            x += c;
            v.push(format!("{} {} {:.2} Balance {:.2}",d[0],d[1],c,b-x));
        }use regex::Regex;

fn balance(s: &str) -> String {
    let re = Regex::new(r"[^\d\w\. ]").unwrap();
    let a = s.split("\n").map(|e| e.to_string()).collect::<Vec<_>>();
    let mut b = 0.0;
    let mut x = 0.0;
    let mut v = vec![];
    for e in a {
        let t = re.replace_all(&e,"").into_owned().trim().to_string();
        if t.is_empty() {continue}
        if v.len()==0 {
            b = t.parse::<f64>().unwrap();
            v.push(format!("Original Balance: {:.2}",b));
        }
        else {
            let c = t.split(" ").last().unwrap().parse::<f64>().unwrap();
            let d = t.split(" ").collect::<Vec<_>>();
            x += c;
            v.push(format!("{} {} {:.2} Balance {:.2}",d[0],d[1],c,b-x));
        }
    }
    v.push(format!("Total expense  {:.2}",x));
    v.push(format!("Average expense  {:.2}",x/(v.len() as f64-2.0)));
    v.join("\n")
}
    }
    v.push(format!("Total expense  {:.2}",x));
    v.push(format!("Average expense  {:.2}",x/(v.len() as f64-2.0)));
    v.join("\n")
}