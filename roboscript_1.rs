use itertools::Itertools;
use regex::*;

pub fn execute(code: &str) -> String {
    let d: Vec<(i32, i32)> = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut p = 0;
    let mut my = 5000;
    let mut ny = 0;
    let mut mx = 5000;
    let mut nx = 0;
    let mut y = 2500;
    let mut x = 2500;
    let mut v = (0..5000)
        .map(|_| (0..5000).map(|_| ' ').collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let s = Regex::new(r"[FLR]\d+")
        .unwrap()
        .replace_all(code, |c: &Captures| {
            let t = c.get(0).unwrap().as_str();
            let n = t[1..].parse::<i32>().unwrap_or(1);
            (0..n).map(|_| t[..1].to_string()).join("")
        });
    for e in (s.to_owned() + "F").chars() {
        v[y][x] = '*';
        my = my.min(y);
        ny = ny.max(y);
        mx = mx.min(x);
        nx = nx.max(x);
        if e == 'F' {
            (y, x) = ((y as i32 + d[p].0) as usize, (x as i32 + d[p].1) as usize);
        } else {
            p = (p + (if e == 'L' { 3 } else { 1 })) % 4;
        }
    }
    v[my..ny + 1]
        .iter()
        .map(|t| t[mx..nx + 1].iter().join(""))
        .join("\r\n")
}
