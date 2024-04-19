use itertools::Itertools;

fn f(a: Vec<u16>) -> u16 {
    let n = a.len() as u16;
    a.iter().sum::<u16>()+n*(n-1)+(0..a.len()).map(|i| a[..i].iter().sum::<u16>()).sum::<u16>()
}

fn barista (o: &[u8], n: usize) -> u16 {
    let mut a = o.iter().cloned().filter(|e| e>&0).sorted().rev().collect::<Vec<_>>();
    let mut t = (0..n).map(|_| if a.len()>0 {a.pop().unwrap()} else {0}).collect::<Vec<_>>();
    let mut r = t.iter().cloned().map(|e| vec![e as u16]).collect::<Vec<_>>();
    while t.iter().any(|e| e>&0) {
        let m = t.iter().cloned().filter(|e| e>&0).min().unwrap();
        t = t.iter().cloned().map(|e| e.saturating_sub(m)).collect();
        for i in 0..t.len() {
            if t[i]==0 {
                t[i] = a.pop().unwrap_or(0);
                if t[i]>0 {r[i].push(t[i] as u16)}
            }
        }
    }
    r.iter().cloned().map(|e| f(e)).sum()
}