use std::collections::HashMap;
use itertools::Itertools;
use regex::Regex;

fn interpreter(code: &str, k: usize, w: usize, h: usize) -> String {
    let s = Regex::new(r"[^nsew\*\[\]]").unwrap().replace_all(code,"");
    let mut d = HashMap::new();
    let mut t = vec![];
    for (i,e) in s.chars().enumerate() {
        if e=='[' {t.push(i);}
        if e==']' {
            let x = t.pop().unwrap();
            d.insert(i,x);
            d.insert(x,i);
        }
    }
    let mut a = (0..h).map(|_| (0..w).map(|_| 0).collect::<Vec<_>>()).collect::<Vec<_>>();
    let mut y = 0;
    let mut x = 0;
    let mut v = 0;
    let mut c = 0;
    while v<s.len() && c<k {
        c += 1;
        let e = s.chars().nth(v).unwrap();
        if e=='n' {y = ((y as isize-1+h as isize) as usize)%h}
        if e=='e' {x = (x+1)%w}
        if e=='s' {y = (y+1)%h}
        if e=='w' {x = ((x as isize-1+w as isize) as usize)%w}
        if e=='*' {a[y][x] ^= 1}
        if e=='[' && a[y][x]==0 {v = *d.get(&v).unwrap()}
        if e==']' && a[y][x]!=0 {v = *d.get(&v).unwrap()}
        v += 1;
    }
    a.iter().map(|t| t.iter().map(|e| e.to_string()).join("")).join("\r\n")
}