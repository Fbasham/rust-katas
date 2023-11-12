use itertools::Itertools;

fn is_prime(n: u32) -> bool {
    if n<2 {return false}
    if n%2==0 {return n==2}
    let mut i=3;
    while i*i<=n {
        if n%i==0 {return false}
        i += 2;
    }
    true
}

fn f(n: u32) -> Vec<Vec<u32>> {
    let mut q = vec![vec![n]];
    for d in 2..=n {
        if n%d==0 && d<=(n as f64).sqrt() as u32 {
            if is_prime(n/d) {
                q.push(vec![d,n/d]);
            }
            else {
                for u in f(n/d) {
                    let mut t: Vec<u32> = u.iter().cloned().collect();
                    t.push(d);
                    if t.iter().product::<u32>()==n {
                        q.push(t);
                    }
                }
            }
        }
    }
    q
}

fn find_spec_prod_part(n: u32, com: &str) -> Option<(Vec<u32>, u32)> {
    if is_prime(n) {return None}
    let mut r = vec![];
    let mut m = if com=="min" {u32::MAX} else {0};
    for t in f(n) {
        if t.len()<2 {continue}
        let d = t.iter().counts();
        let s = d.iter().map(|(k,v)| k.pow(*v as u32)).sum::<u32>()*d.values().map(|e| *e as u32).sum::<u32>();
        match com {
            "min" if s<m => (r,m) = (t,s),
            "max" if s>m => (r,m) = (t,s),
            _ => ()
        }
    }
    Some((r.iter().cloned().sorted().rev().collect(),m))
}