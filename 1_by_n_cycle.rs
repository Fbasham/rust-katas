use std::collections::HashSet;

fn gcd(a: i64, b: i64) -> i64 {
    if b==0 {a} else {gcd(b,a%b)}
}

fn cycle(n: i64) -> i64 {
    if gcd(n,10)!=1 {return -1;}
    let mut s = HashSet::new();
    let mut k = 0;
    let mut d = 1;
    for i in 0.. {
        if i>0 {s.insert(d);}
        d = (d%n)*10;
        if s.contains(&d) {break;}
        k += 1;
    }
    k
}