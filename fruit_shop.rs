use regex::*;

fn fruit_pack(orders: &[&str]) -> Vec<(String, String, String)> {
    let mut r = vec![];
    for s in orders {
        let mut x = String::new();
        let mut y = String::new();
        let mut z = String::new();
        for c in Regex::new(r"\d+.").unwrap().find_iter(s) {
            let t = c.as_str();
            let n = t[..t.len() - 1].parse::<usize>().unwrap();
            let m = &t[t.len() - 1..];
            if n % 10 > 0 {
                x += &format!("({})", m.repeat(n % 10));
            }
            y += &format!("{}", format!("{{{m}}}").repeat((n % 50) / 10));
            z += &format!("{}", format!("[{m}]").repeat(n / 50));
        }
        let k = x.len().max(y.len()).max(z.len());
        x = "-".repeat(k - x.len()) + &x;
        y = "-".repeat(k - y.len()) + &y;
        z = "-".repeat(k - z.len()) + &z;
        r.push((x, y, z));
    }
    r
}
