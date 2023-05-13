use regex::Regex;

fn balance_statement(s: &str) -> String {
    if s == "" {
        return "Buy: 0 Sell: 0".to_string();
    }
    let re = Regex::new(r"^\S+ \d+ \d+\.\d+ [BS]$").unwrap();
    let mut x = 0.0;
    let mut y = 0.0;
    let mut v = vec![];
    for e in s.split(",") {
        if !re.is_match(e.trim()) && e.trim() != "" {
            v.push(e.trim());
        } else {
            let a = e.trim().split(" ").collect::<Vec<_>>();
            let n = a[1].parse::<f64>().unwrap();
            let m = a[2].parse::<f64>().unwrap();
            if a[3] == "B" {
                x += n * m;
            }
            if a[3] == "S" {
                y += n * m;
            }
        }
    }
    if v.is_empty() {
        return format!("Buy: {:.0} Sell: {:.0}", x, y);
    }
    format!(
        "Buy: {:.0} Sell: {:.0}; Badly formed {}: {} ;",
        x,
        y,
        v.len(),
        v.join(" ;")
    )
}
